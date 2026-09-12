# Rapport 2 : Inventaire analytique de Spotify Web API

> Inventaire etabli le 1er septembre 2026 a partir de la documentation et des
> changelogs officiels Spotify. L'objectif est de distinguer metadonnees catalogue,
> etat instantane, preferences algorithmiques et veritable historique statistique.

## Disponibilite des endpoints

| Marqueur | Signification | Avis et consequence |
| --- | --- | --- |
| Actif | Disponible dans le jeu reduit d'endpoints des nouvelles applications Development Mode | Base la plus sure pour un nouveau produit, mais insuffisante pour reproduire stats.fm |
| Dev-26 | Retire du jeu reduit de fevrier 2026 | Ne pas fonder une nouvelle architecture dessus sans confirmation de quota |
| Restreint-24 | Non accessible aux nouveaux cas d'usage depuis le 27 novembre 2024 | A traiter comme dependance historique ou indisponible |
| Deprecie | Toujours documente, mais marque `Deprecated` | Risque eleve de rupture; prevoir degradation explicite |

La presence d'un endpoint dans la navigation Spotify ne garantit pas qu'un nouveau
client puisse l'utiliser. Les applications Extended Quota existantes peuvent avoir un
perimetre plus large. Une analyse doit donc toujours distinguer « champ documente » et
« champ reellement accessible a cette application ».

## Dictionnaire analytique des objets

| Objet | Grain et champs principaux | Ce qu'un statisticien peut mesurer | Ce qu'il ne faut pas conclure | Avis et amelioration |
| --- | --- | --- | --- | --- |
| `AlbumObject` | Une edition d'album; type, nombre de pistes, artistes, images, date, precision, restrictions, IDs | Taille de sortie, anciennete, type, disponibilite, jointure aux pistes | Un album ne correspond pas toujours a une oeuvre canonique; deluxe et remaster peuvent etre separes | Riche pour le catalogue; construire une couche de canonicalisation avant les statistiques |
| `ArtistObject` | Un identifiant artiste; nom, images, genres; historiquement followers et popularity | Diversite d'artistes, etiquettes de genre, portee catalogue historique | Followers et popularity ne mesurent pas l'ecoute personnelle et peuvent etre absents ou obsoletes | Bon axe de jointure; horodater les champs volatils et ne pas les traiter comme mesures utilisateur |
| `TrackObject` | Une version de piste; album, artistes, duree en ms, explicite, ISRC/IDs, restrictions, URI | Duree catalogue, contenu explicite, position dans l'album, deduplication potentielle par ISRC | `duration_ms` n'est pas le temps reellement ecoute; deux IDs peuvent representer le meme enregistrement | Objet central; conserver ID source, ISRC et version canonique separement |
| `PlaylistObject` | Une playlist a un instant; proprietaire, visibilite, snapshot, items | Taille, composition, proprietaire et evolution si des snapshots sont collectes | L'API ne fournit pas automatiquement tout l'historique des modifications | Utile pour un etat; journaliser les snapshots pour obtenir une serie temporelle |
| `PlaylistItemObject` | Une occurrence de contenu dans une playlist; `added_at`, `added_by`, `is_local`, item | Chronologie d'ajouts et contribution entre utilisateurs | Un ajout ne prouve ni lecture ni appreciation | Bonne metrique editoriale; ne pas la melanger aux comportements d'ecoute |
| `UserObject` | Un profil Spotify; identite, image, liens; champs prives historiques selon scopes | Identifier et joindre le proprietaire des donnees | Aucun historique ou profil statistique complet n'est contenu ici | Suffisant pour l'identite, pas pour l'analyse comportementale |
| `PlaybackState` | Etat instantane d'une lecture et d'un appareil | Progression courante, contexte, repeat, shuffle, action autorisee | Une suite de lectures API ponctuelles n'est pas un historique fiable sans collecte continue | Bon temps reel; mauvais substitut a l'export historique |
| `DeviceObject` | Un appareil visible maintenant; type, volume, session privee, actif | Parc d'appareils actuel et contexte instantane | Aucune frequence d'usage historique native | Utile pour telecommande; collecter avec consentement si une tendance est necessaire |
| `PlayHistoryObject` | Un evenement recent; piste, `played_at`, contexte | Ordre et timestamp des ecoutes recentes | Pas de `ms_played`, skip, raison, offline ou historique complet | Trop pauvre pour completion et temps ecoute; utiliser seulement pour synchronisation recente |
| `AudioFeaturesObject` | Une piste; scores acoustiques, BPM, tonalite, mode, loudness, signature | Segmentation sonore, distributions et profil acoustique pondere | Ces scores decrivent la piste, pas l'emotion ou le comportement de l'auditeur | Tres utile analytiquement mais endpoint restreint/deprecie; mettre en cache avec provenance |
| `AudioAnalysisObject` | Une piste decomposee en bars, beats, sections, segments et tatums | Structure temporelle, dynamique, hauteur et timbre | Donnees lourdes et modelisees; pas une verite musicologique | Forte richesse, faible disponibilite; reserver aux cas d'usage musicaux avances |
| `EpisodeObject` | Un episode; show, duree, langues, sortie, progression, restrictions | Catalogue podcast et progression ponctuelle | La progression n'est pas un historique complet de sessions | Utile pour podcasts, hors du coeur musical actuel de stats.fm |
| `ShowObject` | Un podcast; metadonnees, langues, episodes, droits | Diversite de shows et catalogue | Aucun temps d'ecoute cumule sans evenements | Bon catalogue, faible valeur comportementale seul |
| `AudiobookObject` | Un livre; auteurs, narrateurs, edition, chapitres | Diversite d'auteurs, taille et structure | Pas de chronologie complete de lecture | Utile pour une verticale audiobook distincte |
| `ChapterObject` | Un chapitre; numero, duree, progression et livre parent | Avancement et structure | Un `resume_point` courant ne dit pas combien de fois le chapitre a ete ecoute | Correct pour progression, insuffisant pour statistiques historiques |
| `PagingObject<T>` | Une page offset; `limit`, `offset`, `next`, `total`, items | Taille annoncee et parcours d'un ensemble | Une page n'est jamais l'ensemble complet | Toujours paginer et journaliser les erreurs ou elements manquants |
| `CursorPagingObject<T>` | Une page autour d'un curseur temporel ou logique | Parcours stable de flux recents | La couverture temporelle depend du nombre d'evenements, pas seulement de `limit` | Meilleur pour flux; conserver le dernier curseur et detecter les trous |
| `ImageObject` | Une image et dimensions optionnelles | Qualite et format d'affichage | Aucune information statistique musicale | Pure presentation; exclure du modele analytique principal |
| `CategoryObject` | Une categorie editoriale | Taxonomie Spotify et segmentation de browse | Ce n'est pas un genre utilisateur ni un comportement | Utile pour navigation, pas pour profiler directement une personne |

## Unites et nature des champs

| Champ ou famille | Unite / nature | Grain | Interpretation correcte | Avis |
| --- | --- | --- | --- | --- |
| `duration_ms` | Millisecondes | Piste, episode ou chapitre | Duree catalogue du contenu | Toujours separer de `ms_played` |
| `progress_ms` | Millisecondes | Etat courant | Position ponctuelle dans la lecture | Ne pas sommer entre polls sans logique de session |
| `timestamp` | Epoch ms | Etat player | Instant du snapshot | Etat, pas debut prouve de l'ecoute |
| `played_at` | Datetime UTC | Evenement recent | Instant associe a la lecture recente | Convertir avec un fuseau explicite pour jours/heures |
| `added_at` | Datetime | Occurrence de playlist/bibliotheque | Date d'ajout, pas date de premiere ecoute | Bon pour evolution editoriale |
| `release_date` | Date de precision variable | Album/contenu | Date catalogue avec `release_date_precision` | Ne pas inventer jour/mois lorsque seule l'annee existe |
| `popularity` | Score Spotify historique | Piste ou artiste | Signal relatif et volatile de popularite | Horodater; ne pas traiter comme pourcentage ni mesure personnelle |
| `followers` | Compte de followers historique | Artiste/profil | Nombre d'abonnes au moment de la lecture API | Volatile et parfois indisponible; stocker date de collecte |
| Scores Audio Features | Generalement echelles normalisees, sauf champs specifiques | Piste | Descripteurs calcules pour comparer des pistes | Conserver definition et version du fournisseur |
| `loudness` | dB | Piste | Niveau sonore moyen estime | Ne pas convertir implicitement en 0-1 |
| `tempo` | BPM | Piste | Tempo estime | Gerer demi/double tempo dans les analyses |
| `key` / `mode` | Categorie musicale codee | Piste | Tonalite et majeur/mineur estimes | Decoder les codes dans l'UI |
| `time_signature` | Nombre de temps estime par mesure | Piste | Structure metrique | Variable categorielle, pas score ordinal |

## Catalogue des endpoints et valeur statistique

| Categorie | Endpoints principaux | Donnees concretes obtenues | Valeur statistique | Avis et amelioration |
| --- | --- | --- | --- | --- |
| Albums | `GET /albums/{id}`, tracks, bibliotheque; certaines operations Dev-26 | Edition, artistes, tracklist, sortie, restrictions, date d'ajout | Jointure et segmentation du catalogue | Bon enrichissement, aucune mesure d'ecoute sans historique |
| Artists | `GET /artists/{id}`, albums, top tracks Dev-26, related Restreint-24 | Identite, genres, discographie, similarites historiques | Regroupement par artiste et genre | Dependances fragiles; eviter de promettre followers, popularity ou related aux nouveaux clients |
| Audiobooks | Objet, chapitres et bibliotheque | Auteurs, narrateurs, langue, progression | Analyse d'un catalogue parle | A isoler de la musique car unites et comportements differents |
| Categories / Browse | Categories et anciennes playlists editoriales | Taxonomie et contenus editoriaux | Analyse de l'offre Spotify | Faible pour l'historique personnel |
| Chapters | Objet et lot Dev-26 | Numero, duree, livre, progression | Progression structurelle | Pas de temps cumule fiable |
| Episodes | Objet, lot Dev-26 et bibliotheque | Show, duree, sortie, progression | Catalogue podcast et sauvegarde | Necessite un historique exporte pour temps et completion reels |
| Genres | Anciennes graines de recommandation | Liste de labels autorises | Taxonomie de recommandation | Deprecie et non personnalise; ne pas l'utiliser comme distribution utilisateur |
| Library | Sauvegarde, suppression et presence | Etat favori a l'instant de la requete | Taux ecoute/sauvegarde si joint a un historique | Bon signal d'intention, mais l'absence de sauvegarde n'est pas un rejet |
| Markets | `GET /markets` Dev-26 | Codes pays disponibles | Referentiel geographique | Ce n'est pas le pays d'ecoute de chaque evenement |
| Player | Etat, appareils, current, controles, recently played, queue | Etat courant et fenetre recente d'evenements | Temps reel, recence, contexte partiel | Tres utile operationnellement, insuffisant pour statistiques longues |
| Playlists | Lecture/ecriture, items, profil, images | Composition, ordre, ajouts, proprietaires et snapshots | Analyse editoriale et collaborative | Collecter les snapshots pour mesurer les changements |
| Search | Recherche multi-type et filtres | Resultats catalogue | Resolution d'entites et decouverte | La pertinence n'est pas une metrique d'ecoute |
| Shows | Objet, episodes et bibliotheque | Catalogue podcast | Diversite de contenu parle | Pas de comportement sans evenements |
| Tracks | Objet, bibliotheque, Audio Features/Analysis restreints | Version, duree, artistes, album, audio, restrictions | Axe central de jointure et segmentation | Solide pour catalogue; canonicaliser versions et prevoir absence audio |
| Users | `GET /me`, `/me/top/{type}`, profil public Dev-26, following | Identite et ordre algorithmique des tops | Preference relative sur trois horizons | L'ordre n'expose ni score, ni minutes, ni streams; ne pas inventer de poids |

## Ce que `/me/top` signifie statistiquement

`GET /me/top/tracks` et `GET /me/top/artists` produisent un ordre personnalise selon
l'affinite Spotify sur trois horizons algorithmiques. Ils ne fournissent pas :

- le nombre de streams;
- le temps ecoute;
- le score interne qui separe deux rangs;
- un top albums;
- des bornes calendaires exactes;
- une serie temporelle ou un historique evenementiel.

Un rang 1 signifie seulement « premiere entite dans l'ordre retourne pour cet horizon ».
On ne peut pas conclure que l'entite represente une proportion donnee du temps ou deux
fois plus d'ecoutes que le rang 2. Mon avis : cet endpoint est bon pour personnaliser
une interface, mais mauvais pour produire des statistiques quantitatives.

## Ce que `recently-played` signifie statistiquement

`GET /me/player/recently-played` fournit une fenetre limitee d'evenements ordonnes
autour de `played_at`, avec la piste et un contexte partiel. Une limite de 50 evenements
ne represente aucune duree fixe : elle peut couvrir quelques heures pour un utilisateur
tres actif ou plusieurs jours pour un utilisateur occasionnel.

Sans `ms_played`, deux evenements ont le meme poids meme si l'un correspond a 31 secondes
et l'autre a six minutes. Il est impossible d'en tirer proprement :

- temps total ecoute;
- completion;
- skip;
- distribution des durees reelles;
- historique lifetime.

Mon avis : excellent mecanisme de rattrapage recent si le poll est frequent et controle,
mais source dangereuse pour promettre un historique exhaustif.

## Metriques calculables et conditions

| Metrique | Donnees necessaires | Formule ou grain | Faisabilite Web API seule | Avis |
| --- | --- | --- | --- | --- |
| Duree catalogue moyenne | `TrackObject.duration_ms` | Moyenne par piste distincte | Oui | Decrit le catalogue, pas l'ecoute reelle |
| Part explicite | `explicit` et un poids choisi | Somme des poids explicites / somme des poids eligibles | Oui pour catalogue ou fenetre connue | Toujours preciser ponderation par piste ou evenement |
| Age des sorties | `release_date` + precision | Date d'analyse - date de sortie | Oui | Gerer precision annee/mois/jour et reeditions |
| Diversite d'artistes recente | `recently-played.track.artists` | Cardinalite sur la fenetre | Partiellement | Tres sensible a la taille 50 et aux collaborations |
| Recence depuis derniere ecoute | `played_at` | Maintenant - max timestamp visible | Oui sur fenetre | Ne prouve pas la derniere ecoute si la fenetre a un trou |
| Affinite top | Position `/me/top` | Rang ordinal | Oui | Ordinal uniquement, aucune distance entre rangs |
| Profil Audio Features | Features + poids explicite | Moyenne/mediane par piste, stream ou temps | Souvent non pour nouveau client | Preciser poids, manquants et echelles |
| Evolution de playlist | Snapshots collectes | Difference d'items entre dates | Oui avec collecte | Necessite stockage longitudinal propre |
| Temps ecoute reel | `ms_played` evenementiel | Somme par periode/entite | Non | Import Extended Streaming History requis |
| Completion | `ms_played / duration_ms` | Par evenement eligible | Non | Gerer >100 %, repeats et durees nulles |
| Taux de skip | `skipped` renseigne | Skips vrais / evenements avec valeur connue | Non | Disponible dans l'export, pas dans recent API |

## Donnees absentes de l'API Web standard

- Nombre lifetime verifie de lectures d'une piste, d'un album ou d'un artiste.
- Temps lifetime reellement ecoute par entite.
- Historique complet depuis la creation du compte.
- `ms_played`, raisons de debut/fin, skip, offline, shuffle et session privee par evenement.
- Top albums personnel quantitatif.
- Rang d'un utilisateur parmi les auditeurs d'une entite.
- Graphe d'amis stats.fm, Records, Soulmates et charts globaux stats.fm.

Le mot « exact » doit rester prudent : meme un export complet peut contenir des lacunes,
des doublons, des versions de piste differentes et des evenements sans identifiant
catalogue resolvable.

## Avis global sur les donnees Spotify Web API

### Bonnes choses

- Le catalogue est riche, structure et jointable par identifiants.
- Les objets distinguent bien piste, edition d'album, artiste, playlist et contenu parle.
- `played_at`, `added_at`, snapshots et etat player permettent des usages recents ou instantanes.
- Les Audio Features et Audio Analysis, lorsqu'elles sont accessibles, offrent une forte profondeur descriptive.
- Les scopes et restrictions imposent une separation utile entre public, prive et controle du player.

### Mauvaises choses

- L'API standard n'est pas un entrepot d'historique personnel.
- `/me/top` est ordinal et opaque, donc impropre aux volumes quantitatifs.
- `recently-played` manque la duree reelle et couvre un nombre d'evenements, pas une periode garantie.
- Les restrictions 2024 et 2026 rendent plusieurs experiences historiques difficiles a reproduire.
- Les champs volatils ou historiques comme popularity et followers risquent d'etre absents ou incoherents.
- Les IDs suivent souvent une version commerciale, pas une oeuvre canonique.

### Ameliorations recommandees pour un produit

1. Separer dans le modele `catalogue`, `evenement`, `etat instantane` et `agregat`.
2. Stocker provenance, date de collecte, scope et disponibilite de chaque champ.
3. Utiliser l'Extended Streaming History pour les metriques de comportement, avec consentement.
4. Conserver les millisecondes et timestamps bruts; arrondir seulement a l'affichage.
5. Canonicaliser pistes et albums sans supprimer les IDs source.
6. Afficher les valeurs manquantes comme inconnues, jamais comme zero.
7. Documenter denominateur, grain, periode, fuseau et politique de valeurs nulles de chaque KPI.
8. Concevoir une degradation produit lorsque Audio Features, popularity ou related artists sont indisponibles.

## Sources

- [Spotify Web API](https://developer.spotify.com/documentation/web-api)
- [Changements de novembre 2024](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api)
- [Changelog de fevrier 2026](https://developer.spotify.com/documentation/web-api/references/changes/february-2026)
- [Understanding your data](https://support.spotify.com/us/article/understanding-your-data/)
