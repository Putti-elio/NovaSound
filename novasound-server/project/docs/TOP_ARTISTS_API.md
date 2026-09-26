# User Top Artists — Spotify backend

Contract: [`top_artists.proto`](../../code/contracts/proto/top_artists.proto).

## Availability and ownership

**This service is intentionally not registered in the production router.** NovaSound
authentication and Spotify account linking are prerequisites for public exposure.
There is no `user_id`, access token, or refresh token in either RPC request.

All application, import, provider, and storage operations require a non-nil
NovaSound user UUID. A `SpotifyConnection` binds an access token to that user;
using it for another user fails before any HTTP call. The caller that supplies
this connection is trusted to have linked the correct Spotify account. This slice
does not authenticate users, verify account linking, refresh tokens, or persist
credentials. Spotify requires the `user-top-read` scope.

For controlled internal execution, construct `TopArtistsService` with a
`TopArtistsStore`/PostgreSQL pool and call its methods with an explicit `UserId`.
`ConnectTopArtistsService::for_internal_user` binds the same identity for every
request and exists for isolated internal/test servers only. **It is not auth
middleware and must not be added to the user-facing router.** The later auth task
must resolve both identity and connection from verified per-request context.

## GetTopArtists

RPC: `novasound.top_artists.v1.UserTopArtistsService/GetTopArtists`.

Native request (Connect JSON representation):

```json
{
  "provider": "PROVIDER_SPOTIFY",
  "nativePeriod": "NATIVE_PERIOD_SHORT_TERM",
  "limit": 50
}
```

Custom request:

```json
{
  "provider": "PROVIDER_SPOTIFY",
  "customPeriod": {
    "start": "2026-09-01T00:00:00Z",
    "end": "2026-10-01T00:00:00Z"
  },
  "limit": 50
}
```

Specify exactly one period. Missing/unknown providers or native periods are
invalid. `limit` defaults to 50 when omitted/zero; otherwise it must be 1–1000.
Limits are applied **after** ranking/comparison, not before.

### Native affinity

- `SHORT_TERM`, `MEDIUM_TERM`, and `LONG_TERM` map to Spotify's native time ranges.
  These are affinity rankings, not rankings by plays or minutes. Spotify currently
  describes them as approximately four weeks, six months, and one year; clients
  must not interpret them as exact calendar windows or all-time history.
- Each request synchronizes the full available ranking before returning the
  requested display limit. Retrieval uses pages of 50, a maximum of 10,000
  entries, a 20-second per-request timeout, and a 60-second overall deadline.
  Malformed/inconsistent/duplicate pages fail without saving a partial snapshot.
- `rankChange` is previous rank minus current rank within the same user,
  provider, and native period. Positive means moving up. The comparison is the
  previous successful synchronization, not a preceding calendar window.
- The full latest snapshot is saved atomically. A synchronization started before
  a newer saved synchronization cannot overwrite it. A failed request leaves the
  existing snapshot intact. Repeating a successful synchronization compares with
  the last one, so an unchanged ranking reports zero.
- `metrics`, `historyStart`, and `historyEnd` are absent: Spotify does not provide
  these listening statistics through `/me/top/artists`. Imported metrics are not
  attached by matching names or presented as native Spotify data.

### Custom imported-history ranking

- UTC instants use RFC 3339; offsets are normalized. The interval is **[start,
  end)**, and each event is assigned by its export end timestamp. The start must
  precede the end. Current and preceding windows must fit in years 1970–9999.
  Timestamps support PostgreSQL's microsecond precision; sub-microsecond values
  and leap-second representations are rejected rather than silently rounded.
- Rank by **all imported music play events**, including zero-duration and short
  events. Ties sort by stable internal artist UUID ascending; ranks are sequential.
- `playCount`, `playedMs`, and `streams30s` describe the requested window.
  `streams30s` counts events with `ms_played >= 30000`, not estimated full listens.
  It is an application metric, not Spotify royalty/analytics data.
- `firstPlay`/`lastPlay` are the first/last known events for that artist across all
  imported history, including outside the requested window.
- `rankChange` compares against the immediately preceding equal-length window,
  using the same ranking rule and import snapshot. An artist without any event in
  the comparison window has no rank change; it is not assigned a fictitious rank.
- `historyStart`/`historyEnd` bound the user's imported events. **They do not prove
  complete coverage between those instants**, and no full-history claim is made.
  Empty imports/results have no observed bounds. Counts mean “observed in imports,”
  not “everything ever played on Spotify.”

### Entry and provenance fields

| Field | Meaning / absence |
| --- | --- |
| `rank` | One-based rank in the complete result |
| `artist.id` | Stable NovaSound UUID, scoped to this user; never a Spotify ID |
| `artist.name` | Source artist display name |
| `artist.imageUrl` | Optional HTTPS provider image; absent when unavailable |
| `artist.providers[]` | Provider associations with optional external artist ID |
| `metrics` | Present only for calculated imported-history results |
| `rankChange` | Signed delta; absent without a comparable prior entry |
| `source` | `NATIVE_AFFINITY` or `IMPORTED_HISTORY`, on each entry and response |
| `observedAt` | Native synchronization start, or custom query observation time |
| `comparisonAt` | Prior native synchronization start, or preceding custom window start |

`IMPORTED_HISTORY` currently means Spotify Extended Streaming History (ESH).
Account Data, recent-play Web API data, and Technical Log Information are not
imported by this feature. Provider payloads do not leak through this contract.

Proto3 optional fields/message presence are meaningful. In Connect JSON, absent
values are omitted, not replaced by zero; all `int64`/`uint64` fields serialize
as decimal strings. Do not display a missing metric or rank change as zero.

### Artist identity limitations

ESH contains the album artist's name and a track URI, **not a Spotify artist ID**.
ESH artists therefore have stable name-based identities scoped to user/provider,
no external artist ID, and no image. Exact source names group together; artist
renames may split groups and namesakes may share a group. Native artists use
provider artist IDs and a different identity namespace. No guessed or fuzzy
merge is performed between ESH names, native artists, or the existing global
catalogue. Multi-artist track credits are not inferred from ESH album artist names.
Authoritative track-to-artist enrichment/identity reconciliation is outside this
slice; consumers should show imported identities as unresolved when the provider
artist ID is absent.

## ImportListeningHistory

RPC: `novasound.top_artists.v1.UserTopArtistsService/ImportListeningHistory`.

```json
{
  "provider": "PROVIDER_SPOTIFY",
  "format": "HISTORY_FORMAT_JSON",
  "content": "<base64 encoded ESH JSON bytes>"
}
```

`HISTORY_FORMAT_ZIP` accepts Spotify ESH archives. JSON accepts one export array;
multiple files may be imported separately. ZIP supports nested
`Streaming_History_Audio_*.json` and legacy `endsong_*.json` members. Unrelated
members (including Technical Log Information) are not read. Paths escaping the
archive root, corrupt/encrypted/unsupported archives, invalid JSON, invalid dates,
negative/oversized durations, and invalid music metadata fail the import. Podcast,
audiobook, and otherwise unidentifiable music rows are counted as `skipped`.
Local music with track/artist names but no Spotify track URI is accepted.

- Max compressed/direct input: **32 MiB**.
- Max selected expanded ZIP contents: **128 MiB**.
- Max archive members: **1,000**; max total records: **250,000** per import.
- At most **two imports per shared application service** run concurrently;
  additional imports receive `import_busy` rather than accumulating in memory.
- Parsing/decompression runs off the async executor. ZIP contents are never
  extracted to disk. No raw export, IP/device/payment/account fields, or credentials
  are stored. Only normalized artist metadata, event fingerprint, UTC end instant,
  and played milliseconds are persisted.
- The entire import is transactional. Repeated or overlapping files/ZIPs are
  idempotent within each user/provider, including concurrent imports. Different
  users can import identical events independently.
- Deduplication fingerprints UTC end timestamp + track URI + played milliseconds.
  Local music uses artist/track/album names instead of the URI. Metadata-only
  corrections to a Spotify track do not create new events. Spotify exports lack
  unique play IDs: indistinguishable events collapse, and changed timestamps,
  durations, or local-track metadata cannot be reconciled automatically.
- Response counters: `inserted`, `duplicates`, `skipped`. Nothing is partially
  inserted when validation fails. A cancelled write may commit; retry is safe.

The future authenticated upload transport must enforce its wire-body limit before
decoding and configure Connect's receive limit for the desired upload size (JSON
base64 is larger than raw bytes). The current unregistered adapter does not widen
the shared server's default body limits. Large exports can be split into valid ESH
arrays and imported separately through the internal application entry point.

## Stable errors

Errors include detail type `type.novasound.dev/top-artists-error`, with a `debug`
object containing `code` and nullable `retry_after_seconds`.

| Detail code | Connect code | Action |
| --- | --- | --- |
| `invalid_input`, `invalid_import` | `invalid_argument` | Correct input |
| `connection_required` | `failed_precondition` | Supply/relink a valid connection |
| `access_denied` | `permission_denied` | Correct user/scope/provider authorization |
| `rate_limited` | `resource_exhausted` | Honor optional upstream retry delay |
| `import_too_large`, `import_busy` | `resource_exhausted` | Split input / retry later |
| `provider_unavailable`, `invalid_provider_response` | `unavailable` | Retry later; no partial snapshot saved |
| `stale_synchronization` | `aborted` | A newer synchronization already won |
| `internal` | `internal` | Storage/worker failure; no internal details exposed |

No automatic token refresh or HTTP retry loop is introduced. Provider bodies and
credentials are never included in public errors.

## Extension and verification

`TopArtistsProvider` accepts `UserId` and `NativePeriod` and returns normalized
`NativeRanking`. A future YouTube Music adapter must validate this seam with real
capabilities rather than imitate Spotify wire formats. Add supported providers and
their explicit transport mapping then; do not implement frontend provider parsing.
The Spotify-specific history decoder remains in the provider crate.

Sources verified during implementation:
- [Spotify Get User's Top Items](https://developer.spotify.com/documentation/web-api/reference/get-users-top-artists-and-tracks)
- [Spotify data export explanation](https://support.spotify.com/article/understanding-my-data/)

Tests use mocked HTTP/provider inputs and real PostgreSQL. No live user credentials
or private exports are needed. Run `make generate` after contract/query changes,
then `make lint`, `make check-backend`, and `make test` inside the Docker workflow.
