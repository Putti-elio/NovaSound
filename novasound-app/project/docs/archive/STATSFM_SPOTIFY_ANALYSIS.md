# Audit stats.fm et inventaire Spotify Web API

> **Document remplacé.** Cette synthèse préliminaire est remplacée par le
> [Rapport 1 : audit UI stats.fm](../research/statsfm/01_STATSFM_UI_AUDIT.md), le
> [Rapport 2 : inventaire Spotify Web API](../../../../novasound-server/project/docs/research/providers/02_SPOTIFY_API_INVENTORY.md)
> et le [Rapport 3 : corrélation et données manquantes](../research/spotify/03_CORRELATION_GAP_ANALYSIS.md).

> Analyse realisee le 1er septembre 2026 a partir des pages publiques de stats.fm,
> de la documentation Spotify Web API et des pages de support officielles.
>
> Etat du document : rapport initial. Le Rapport 1 doit encore etre complete par
> une exploration authentifiee du profil `https://stats.fm/stenguyz`.

## Methodologie et limites

- **Observe** : verifie directement avec Playwright dans l'interface publique.
- **Documente** : confirme par une source officielle stats.fm ou Spotify.
- **Authentification requise** : composant visible, mais donnees personnelles masquees sans connexion.
- **Restreint** : endpoint documente, mais non accessible aux nouvelles applications ou au Development Mode selon les regles Spotify actuelles.
- Aucune donnee masquee derriere une connexion n'est inventee dans ce rapport.
- Certaines captures promotionnelles de stats.fm peuvent correspondre a une version mobile anterieure.

# Rapport 1 : Audit UI et donnees de stats.fm

## Visualisations recensees

| Page / vue UI | Nom / type de visualisation | Donnees affichees | Statut |
| --- | --- | --- | --- |
| Landing page | Hero et captures produit | Top tracks, artistes, albums, historique et statistiques avancees | Observe |
| Landing page | Compteurs globaux | Utilisateurs, utilisateurs Plus, streams, pistes, artistes et albums connus par stats.fm | Observe, proprietaire stats.fm |
| Accueil applicatif | Carte Now Playing | Piste en cours, artiste, album et pochette | Documente par captures officielles |
| Accueil applicatif | Listes et carrousels | Top artistes, charts globaux et pistes recemment ecoutees | Documente |
| Profil | Cartes KPI | `streams`, `minutes streamed`, `hours streamed`, `different tracks`, `different artists`, `different albums` | Observe |
| Profil | Comparaison de periode | Evolution en pourcentage par rapport a une periode precedente | Documente |
| Profil | Bar chart Top genres | Genres classes selon l'activite d'ecoute | Observe et documente |
| Profil | Classement Top tracks | Rang, titre, artiste, pochette, minutes et streams | Observe |
| Profil | Classement Top artists | Rang, artiste, minutes et streams | Observe |
| Profil | Classement Top albums | Rang, album, pochette, minutes et streams | Observe |
| Profil | Grille ou liste | Presentation alternative des classements | Observe |
| Profil | Listening clocks radiales | Repartition horaire des streams et minutes sur 24 heures | Observe |
| Profil | Timeline Recent streams | Date, heure relative, piste, artiste, album et pochette | Observe |
| Profil | Historique complet | Evenements groupes par date et duree reellement ecoutee, par exemple `listened for 2:11` | Observe |
| Profil | Selecteur temporel | 4 semaines, 6 mois, lifetime et periodes personnalisees | Observe et documente |
| Profil | Methode de classement | Machine learning Spotify, nombre de streams ou temps d'ecoute | Documente |
| Profil | Informations sociales | Avatar, nom, pronoms, biographie, liens et nombre d'amis | Observe |
| Profil | Soulmates et comparaison | Pourcentage de compatibilite, titres communs et comparaison des ecoutes | Documente |
| Profil | Records | Records or, argent et bronze associes aux classements d'auditeurs | Documente |
| Artiste | Cartes de metadonnees | Followers Spotify, popularite sur 10 et genres | Observe |
| Artiste connecte | Cartes KPI personnelles | Streams, minutes, premier stream, dernier stream et rang personnel | Authentification requise |
| Artiste | Top tracks | Catalogue des pistes principales | Observe |
| Artiste | Top albums | Albums et singles | Observe |
| Artiste | Related artists | Artistes similaires | Observe |
| Artiste | Top listeners | Utilisateurs classes par temps d'ecoute et streams | Authentification requise |
| Piste | Cartes de metadonnees | Popularite sur 10, duree, artiste et album | Observe |
| Piste connectee | Cartes KPI personnelles | Nombre personnel de streams et minutes ecoutees | Authentification requise |
| Piste | Barres Audio features | `danceability`, `energy`, `speechiness`, `acousticness`, `instrumentalness`, `liveness`, `valence` | Observe |
| Piste | Valeurs audio | `loudness`, `key`, `mode`, `time_signature`, `tempo` / BPM | Observe |
| Piste | Top listeners | Auditeurs classes selon leur activite sur la piste | Authentification requise |
| Album | Cartes de metadonnees | Nombre de pistes, popularite, `album_type` et date de sortie | Observe |
| Album connecte | Cartes KPI personnelles | Streams, minutes, premier stream et dernier stream | Authentification requise |
| Album | Tracklist | Numero de piste, titre et artistes | Observe |
| Album | Top listeners | Auditeurs classes par temps et streams | Authentification requise |
| Stats applicatives | Serie temporelle | Streams et minutes par jour, semaine, mois ou periode personnalisee | Documente |
| Stats applicatives | Cartes de moyenne | Streams moyens, minutes moyennes et totaux distincts | Documente |
| Stats applicatives | Resume audio agrege | Moyenne ou distribution des caracteristiques audio ecoutees | Documente |
| Charts globaux | Classements | Artistes, pistes et albums classes par les streams enregistres par stats.fm | Documente, calcul proprietaire |
| Playlists | Resume statistique | Ecoutes, minutes, popularite et caracteristiques audio des pistes | Documente |
| Import | Etat d'import | Fichiers, progression, traitement et erreurs | Documente |

## Exemple public observe

Le profil public de demonstration affichait, pour les quatre dernieres semaines :

| Metrique | Valeur observee |
| --- | ---: |
| Streams | 2 506 |
| Minutes ecoutees | 6 455 |
| Heures ecoutees | 108 |
| Pistes distinctes | 1 624 |
| Artistes distincts | 603 |
| Albums distincts | 991 |

Ces valeurs changent continuellement et ne constituent pas des constantes produit.

## Limites de l'observation publique

- Les totaux personnels des pages Artiste, Piste et Album affichent `Login to see`.
- Les classements `Top listeners` sont presents mais masques sans connexion.
- stats.fm annonce des `Advanced charts` sans publier de catalogue exhaustif et versionne.
- Les podcasts et fichiers locaux sont actuellement exclus des statistiques affichees.
- Le profil `stenguyz` sera audite apres authentification pour completer cette section.

# Rapport 2 : Inventaire des donnees Spotify Web API

## Disponibilite des endpoints

| Marqueur | Signification |
| --- | --- |
| Actif | Disponible dans le jeu reduit d'endpoints des nouvelles applications Development Mode |
| Dev-26 | Retire du jeu reduit de fevrier 2026; disponibilite possible pour certaines integrations historiques ou Extended Quota |
| Restreint-24 | Non accessible aux nouveaux cas d'usage depuis le 27 novembre 2024 |
| Deprecie | Toujours documente, mais marque `Deprecated` |

La presence d'un endpoint dans la navigation Spotify ne garantit pas qu'un nouveau
client puisse l'utiliser. Les applications Extended Quota existantes peuvent disposer
d'un perimetre plus large.

## Objets principaux

| Objet | Champs bruts principaux |
| --- | --- |
| `AlbumObject` | `album_type`, `total_tracks`, `external_urls`, `href`, `id`, `images`, `name`, `release_date`, `release_date_precision`, `restrictions`, `type`, `uri`, `artists`, `tracks`, `copyrights`, `external_ids`, `genres` |
| `ArtistObject` | `external_urls`, `genres`, `href`, `id`, `images`, `name`, `type`, `uri`; historiquement `followers`, `popularity` |
| `TrackObject` | `album`, `artists`, `disc_number`, `duration_ms`, `explicit`, `external_ids`, `external_urls`, `href`, `id`, `is_playable`, `restrictions`, `name`, `preview_url`, `track_number`, `type`, `uri`, `is_local` |
| `PlaylistObject` | `collaborative`, `description`, `external_urls`, `href`, `id`, `images`, `name`, `owner`, `public`, `snapshot_id`, `type`, `uri`, `items` |
| `PlaylistItemObject` | `added_at`, `added_by`, `is_local`, `item` |
| `UserObject` | `account_id`, `display_name`, `external_urls`, `href`, `id`, `images`, `type`, `uri`; historiquement `country`, `email`, `explicit_content`, `followers`, `product` |
| `PlaybackState` | `device`, `repeat_state`, `shuffle_state`, `context`, `timestamp`, `progress_ms`, `is_playing`, `item`, `currently_playing_type`, `actions.disallows` |
| `DeviceObject` | `id`, `is_active`, `is_private_session`, `is_restricted`, `name`, `type`, `volume_percent`, `supports_volume` |
| `PlayHistoryObject` | `track`, `played_at`, `context` |
| `AudioFeaturesObject` | `acousticness`, `analysis_url`, `danceability`, `duration_ms`, `energy`, `id`, `instrumentalness`, `key`, `liveness`, `loudness`, `mode`, `speechiness`, `tempo`, `time_signature`, `track_href`, `type`, `uri`, `valence` |
| `AudioAnalysisObject` | `meta`, `track`, `bars`, `beats`, `sections`, `segments`, `tatums`; les segments incluent notamment `pitches`, `timbre` et `loudness_*` |
| `EpisodeObject` | `audio_preview_url`, `description`, `html_description`, `duration_ms`, `explicit`, `external_urls`, `href`, `id`, `images`, `is_externally_hosted`, `is_playable`, `languages`, `name`, `release_date`, `resume_point`, `restrictions`, `show`, `uri` |
| `ShowObject` | `copyrights`, `description`, `html_description`, `explicit`, `external_urls`, `href`, `id`, `images`, `is_externally_hosted`, `languages`, `media_type`, `name`, `total_episodes`, `episodes`, `uri` |
| `AudiobookObject` | `authors`, `copyrights`, `description`, `html_description`, `edition`, `explicit`, `external_urls`, `href`, `id`, `images`, `languages`, `media_type`, `name`, `narrators`, `total_chapters`, `chapters`, `uri` |
| `ChapterObject` | `audiobook`, `audio_preview_url`, `chapter_number`, `description`, `duration_ms`, `explicit`, `images`, `is_playable`, `languages`, `name`, `release_date`, `resume_point`, `restrictions`, `uri` |
| `PagingObject<T>` | `href`, `limit`, `next`, `offset`, `previous`, `total`, `items` |
| `CursorPagingObject<T>` | `cursors.after`, `cursors.before`, `href`, `limit`, `next`, `total`, `items` |
| `ImageObject` | `url`, `height`, `width` |
| `CategoryObject` | `href`, `icons`, `id`, `name` |

## Catalogue des endpoints

| Categorie | Endpoints principaux | Donnees exploitables |
| --- | --- | --- |
| Albums | `GET /albums/{id}`, `GET /albums` (Dev-26), `GET /albums/{id}/tracks`, `GET /me/albums`, anciennes operations `/me/albums`, `GET /browse/new-releases` (Dev-26) | Metadonnees, artistes, images, sortie, type, tracklist, copyrights, IDs externes, restrictions et date d'ajout |
| Artists | `GET /artists/{id}`, `GET /artists` (Dev-26), `GET /artists/{id}/albums`, `GET /artists/{id}/top-tracks` (Dev-26), `GET /artists/{id}/related-artists` (Restreint-24) | Identite, images, genres, albums, top tracks, artistes similaires, followers et popularite historiques |
| Audiobooks | `GET /audiobooks/{id}`, `GET /audiobooks` (Dev-26), `GET /audiobooks/{id}/chapters`, `GET /me/audiobooks`, anciennes operations de bibliotheque | Auteurs, narrateurs, descriptions, edition, langues, chapitres, progression et bibliotheque |
| Categories / Browse | `GET /browse/categories`, `GET /browse/categories/{id}` (Dev-26), featured et category playlists (Restreint-24) | Categories editoriales, icones, noms et playlists associees |
| Chapters | `GET /chapters/{id}`, `GET /chapters` (Dev-26) | Numero, duree, audiobook, description, langues, sortie, progression et restrictions |
| Episodes | `GET /episodes/{id}`, `GET /episodes` (Dev-26), `GET /me/episodes`, anciennes operations de bibliotheque | Show parent, duree, description, sortie, images, progression et statut explicite |
| Genres | `GET /recommendations/available-genre-seeds` (Deprecie, Restreint-24) | Tableau de genres utilisables comme graines de recommandation |
| Library | `PUT /me/library`, `DELETE /me/library`, `GET /me/library/contains` | Sauvegarde, suppression et presence d'URI dans la bibliotheque |
| Markets | `GET /markets` (Dev-26) | Codes pays ISO 3166-1 alpha-2 |
| Player | `GET /me/player`, transfert, appareils, currently playing, controles, recently played, queue | Etat de lecture, appareil, volume, contexte, progression, timestamp, repeat, shuffle, 50 ecoutes recentes et file d'attente |
| Playlists | `GET /playlists/{id}`, `PUT /playlists/{id}`, operations `/items`, `GET /me/playlists`, `POST /me/playlists`, images; anciens chemins `/tracks` deprecies | Nom, description, proprietaire, visibilite, collaboration, image, snapshot, elements, ordre, date et auteur d'ajout |
| Search | `GET /search` | Albums, artistes, playlists, pistes, shows, episodes et audiobooks; filtres `album`, `artist`, `track`, `year`, `upc`, `isrc`, `genre` et tags |
| Shows | `GET /shows/{id}`, `GET /shows` (Dev-26), `GET /shows/{id}/episodes`, `GET /me/shows`, anciennes operations de bibliotheque | Metadonnees de podcast, langues, images, droits, episodes et date de sauvegarde |
| Tracks | `GET /tracks/{id}`, `GET /tracks` (Dev-26), `GET /me/tracks`, Audio Features, Audio Analysis et Recommendations (Deprecies, Restreints-24) | Metadonnees, duree, explicite, IDs externes, restrictions, sauvegarde, proprietes audio, structure musicale et recommandations |
| Users | `GET /me`, `GET /me/top/{type}`, `GET /users/{id}` (Dev-26), `GET /me/following`, anciennes operations follow | Profil, image, identifiants, top artistes ou pistes et artistes suivis |

## Donnees absentes de l'API Web standard

- Nombre lifetime exact de lectures d'une piste, d'un album ou d'un artiste.
- Temps lifetime exact ecoute par entite.
- Historique complet depuis la creation du compte.
- `ms_played` pour chaque evenement de `recently-played`.
- Raisons de debut et de fin d'une ecoute.
- Historique des skips, ecoutes hors ligne et sessions privees.
- Top albums personnel direct.
- Rang d'un utilisateur parmi les auditeurs d'un artiste.
- Amis stats.fm, records, Soulmates et charts globaux stats.fm.

`GET /me/player/recently-played` ne retourne qu'une fenetre limitee, generalement
les 50 dernieres pistes, avec `track`, `played_at` et `context`.

# Rapport 3 : Matrice de correlation et Gap Analysis

## Donnees Spotify exploitees par stats.fm

| Visualisation stats.fm | Source Spotify | Transformation stats.fm |
| --- | --- | --- |
| Profil | `GET /me` | Association au compte et ajout de donnees sociales stats.fm |
| Top artistes et pistes sans import | `GET /me/top/{type}` | Presentation du classement Spotify sur trois fenetres |
| Ecoutes recentes | `GET /me/player/recently-played` | Stockage periodique, deduplication et timeline |
| Now Playing | `GET /me/player/currently-playing` ou `GET /me/player` | Carte temps reel |
| Metadonnees Piste | `GET /tracks/{id}` | Duree, album, artistes, artwork et liens |
| Metadonnees Album | `GET /albums/{id}` et `/tracks` | Tracklist, type, date et images |
| Metadonnees Artiste | `GET /artists/{id}` | Nom, images et genres |
| Top tracks d'un artiste | `GET /artists/{id}/top-tracks` | Liste publique; endpoint retire du nouveau Dev Mode |
| Related artists | `GET /artists/{id}/related-artists` | Liste publique; endpoint restreint depuis 2024 |
| Followers et popularite | Champs historiques `followers` et `popularity` | Normalisation visible de la popularite sur 10 |
| Audio features | `/audio-features/{id}` | Barres et valeurs audio |
| Top genres | `ArtistObject.genres` | Agregation ponderee selon les ecoutes |
| Playlists | Endpoints playlist et `/items` | Agregation des pistes de la playlist |

Les visualisations basees sur `popularity`, `followers`, Related Artists, Audio
Features, Audio Analysis ou Artist Top Tracks dependent aujourd'hui de champs ou
endpoints restreints. Leur presence dans stats.fm peut reposer sur un acces historique,
un quota etendu ou des donnees mises en cache.

## Donnees non representees et opportunites

| Opportunite | Donnees disponibles | Visualisation possible | Limite |
| --- | --- | --- | --- |
| Structure d'une piste | `bars`, `beats`, `sections`, `segments`, `tatums` | Timeline musicale et densite rythmique | Audio Analysis restreinte |
| Profil harmonique | `pitches`, `timbre`, `key`, `mode` | Chromagramme et carte tonale | Endpoint restreint |
| Dynamique | Champs `loudness_*` | Courbe de dynamique | Endpoint restreint |
| Appareils | `device.type`, `name`, `volume_percent`, session privee | Repartition par appareil | Etat instantane, pas historique |
| Contextes | `context.type`, `context.uri` | Repartition album, playlist ou artiste | Historique recent incomplet |
| Playback | Repeat, shuffle, progression et actions interdites | Tableau de bord temps reel | Peu pertinent sans collecte continue |
| Bibliotheque | `added_at` et objets sauvegardes | Evolution et anciennete des favoris | Non observe graphiquement |
| Artistes suivis | `GET /me/following` | Suivis compares aux ecoutes reelles | Non observe |
| Playlists | `added_at`, `added_by`, `owner`, `snapshot_id` | Contributions et evolution | Acces limite aux playlists autorisees |
| Catalogue | Sortie, type d'album, disque et numero de piste | Decennies preferees et chronologie | Non observe |
| Contenu explicite | `explicit` | Part explicite / non explicite | Non observe |
| Duree theorique | `duration_ms` | Distribution de longueur | Non observe |
| Podcasts | Shows, Episodes et `resume_point` | Temps podcast et progression | Exclus par stats.fm actuellement |
| Audiobooks | Livres, chapitres, auteurs et narrateurs | Progression et top auteurs | Aucun graphique observe |
| Recommendations | Seeds, cibles audio et resultats | Carte de decouverte | Endpoint deprecie et restreint |

## Donnees proprietaires ou calculees

| Metrique stats.fm | Calcul ou origine | Motif |
| --- | --- | --- |
| Streams lifetime | Comptage des evenements importes et synchronises | Aucun compteur personnel Spotify |
| Minutes et heures lifetime | Somme de `ms_played` | Absent de `recently-played` |
| Premier et dernier stream | Minimum et maximum des timestamps | Absent de l'API |
| Entites distinctes | Deduplication des URI | Agregat stats.fm |
| Top albums personnel | Regroupement des streams par album | `/me/top` ne supporte pas les albums |
| Tri par streams | Comptage evenementiel | L'API fournit seulement son ordre personnalise |
| Tri par temps | Somme de `ms_played` | Agregat stats.fm |
| Periodes personnalisees | Filtrage de l'historique local | `/me/top` propose trois fenetres |
| Listening clocks | Agregation de l'heure, du nombre et de la duree | Aucun endpoint equivalent |
| Series temporelles | Regroupement par jour, semaine ou mois | Calcul stats.fm |
| Pourcentages d'evolution | Comparaison de periodes | Calcul stats.fm |
| Top genres | Jointure streams, artistes et genres | Pas de top genres utilisateur natif |
| Resume audio | Jointure streams et Audio Features | Calcul stats.fm |
| Top listeners | Comparaison des utilisateurs stats.fm | Donnees privees absentes de Spotify |
| Records | Rang historique dans les leaderboards | Donnee proprietaire |
| Soulmates | Similarite entre profils | Algorithme proprietaire |
| Charts globaux | Agregation de tous les streams stats.fm | Different des charts Spotify |
| Rank movement | Comparaison des classements | Calcul stats.fm |
| Duree d'un evenement | `ms_played` importe | Non expose par l'API standard |
| Detection des skips | `skipped`, `reason_end` et seuil de duree | Absent de `recently-played` |
| Deduplication | Rapprochement timestamps et URI | Traitement stats.fm |

## Extended Streaming History

L'export Spotify Extended Streaming History contient notamment :

- `ts`
- `username`
- `platform`
- `ms_played`
- `conn_country`
- `ip_addr`
- `user_agent_decrypted`
- `master_metadata_track_name`
- `master_metadata_album_artist_name`
- `master_metadata_album_album_name`
- `spotify_track_uri`
- Les metadonnees d'episode et de show
- `reason_start`
- `reason_end`
- `shuffle`
- `skipped`
- `offline`
- `offline_timestamp`
- `incognito_mode`

stats.fm utilise cet import pour obtenir les compteurs exacts, les periodes
personnalisees, l'historique complet et les listening clocks. Le service recupere
ensuite periodiquement les 50 dernieres pistes et les fusionne avec l'import.

Le support indique environ 60 minutes sur la page de synchronisation et environ
100 minutes sur la page generale d'import. Il faut retenir une synchronisation
approximativement horaire, pas un delai garanti.

stats.fm precise egalement que :

- Les ecoutes de moins de 30 secondes sont conservees mais exclues des statistiques.
- Les pistes marquees comme skippees sont exclues.
- Les doublons entre import et synchronisation sont filtres.
- Les fichiers locaux et podcasts sont exclus.
- La synchronisation seule ne garantit pas des statistiques lifetime exactes.
- L'import complet fait partie de stats.fm Plus.

# Conclusion

stats.fm combine trois couches :

1. Le catalogue Spotify pour les artistes, albums, pistes, images, genres et proprietes audio.
2. La personnalisation Spotify pour les top tracks/artists, le playback courant et les ecoutes recentes.
3. Une base evenementielle stats.fm construite par import, synchronisation, deduplication et aggregation.

Les compteurs exacts, minutes lifetime, periodes personnalisees, listening clocks,
top albums, leaderboards et Soulmates ne peuvent pas etre reproduits fidelement avec
la seule Spotify Web API standard.

# Sources

- [stats.fm](https://stats.fm/)
- [Profil public de demonstration](https://stats.fm/sjoerdgaatwakawaka)
- [Historique public](https://stats.fm/sjoerd/streams)
- [stats.fm Plus](https://stats.fm/plus)
- [Support stats.fm : import](https://support.stats.fm/docs/import/)
- [Support stats.fm : methodes de calcul](https://support.stats.fm/docs/import/faq/calculation-methods)
- [Support stats.fm : synchronisation](https://support.stats.fm/docs/streams/sync)
- [Spotify Web API](https://developer.spotify.com/documentation/web-api)
- [Spotify : changements de novembre 2024](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api)
- [Spotify : changelog de fevrier 2026](https://developer.spotify.com/documentation/web-api/references/changes/february-2026)
- [Spotify : Understanding your data](https://support.spotify.com/us/article/understanding-your-data/)
