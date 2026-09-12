# Rapport 3 : Corrélation Spotify ↔ stats.fm et analyse des données manquantes

> Version remaniée le 5 septembre 2026. Cette analyse distingue les données obtenues par la Spotify Web API, celles issues de l’export personnel Spotify, celles calculées par stats.fm et celles qui restent inexploitées ou indisponibles.

## 1. Réponse synthétique

**Non, stats.fm n’utilise pas 100 % des possibilités offertes par Spotify.**

Le produit exploite déjà une grande partie des données utiles à son objectif principal : goûts musicaux, tops, écoutes récentes, métadonnées du catalogue et statistiques avancées. Mais ses statistiques historiques exactes ne peuvent pas venir de la seule Spotify Web API : elles reposent largement sur l’**Extended Streaming History** exporté par l’utilisateur, puis sur des calculs propres à stats.fm.

| Périmètre | Couverture fonctionnelle estimée | Lecture |
| --- | ---: | --- |
| Données nécessaires aux statistiques musicales personnelles | Élevée, environ 75–90 % | tops, historique importé, temps d’écoute et agrégations sont bien couverts |
| Spotify Web API dans son ensemble | Partielle, environ 40–60 % | bibliothèque, playlists, podcasts, livres audio, appareils et contrôle du lecteur sont moins exploités |
| Historique exact accessible par la Web API seule | Faible | ni historique complet ni durée réellement écoutée par événement |
| Potentiel analytique encore disponible | Important | surtout via export + catalogue + bibliothèque + playlists |

Ces fourchettes expriment une couverture fonctionnelle, pas une mesure officielle du code interne de stats.fm.

## 2. Les quatre origines d’une information

| Origine | Définition | Exemples |
| --- | --- | --- |
| Spotify Web API directe | Donnée renvoyée par un endpoint OAuth ou catalogue | profil, piste, album, artiste, tops Spotify, lecture en cours, écoutes récentes |
| Export personnel Spotify | Fichier demandé par l’utilisateur, extérieur à la Web API | timestamp, `ms_played`, plateforme, pays, skip, hors ligne, raisons de début/fin |
| Calcul stats.fm | Agrégat produit à partir des événements stockés | minutes totales, tops par temps, premier stream, horloges, séries temporelles |
| Donnée propriétaire stats.fm | Information créée grâce à la communauté ou aux règles du service | Soulmates, amis, records, Top listeners, classements globaux |

Une donnée visible dans stats.fm n’est donc pas nécessairement récupérée directement par l’API Spotify.

## 3. Matrice de corrélation : source et transformation

| Fonction stats.fm | Source primaire | Données brutes | Transformation stats.fm | Nature |
| --- | --- | --- | --- | --- |
| Connexion et identité | `GET /me` | `account_id`, nom, image, liens | association au compte stats.fm et profil social | Spotify + propriétaire |
| Top artistes | `GET /me/top/artists` ou historique | affinité Spotify sur trois périodes, ou événements | classement par affinité, streams ou temps | Mixte |
| Top titres | `GET /me/top/tracks` ou historique | affinité Spotify ou événements | classement par affinité, streams ou temps | Mixte |
| Top albums personnel | historique + métadonnées de piste | album rattaché à chaque écoute | regroupement des événements par album | Calcul stats.fm |
| Top genres | genres artistes + historique | artistes, genres et poids d’écoute | jointure et agrégation pondérée | Calcul stats.fm |
| Lecture en cours | `/me/player/currently-playing` ou `/me/player` | piste, progression, contexte, état | présentation temps réel | Spotify direct |
| Écoutes récentes | `/me/player/recently-played` | piste, `played_at`, contexte, max. 50 éléments | collecte, stockage, déduplication | Spotify + traitement |
| Historique complet | Extended Streaming History | timestamp, URI, noms, `ms_played` | import, validation, déduplication | Export + traitement |
| Nombre de streams | historique et synchronisation | événements d’écoute | règles d’éligibilité puis comptage | Calcul stats.fm |
| Minutes/heures écoutées | Extended Streaming History | `ms_played` | somme, conversion, arrondi | Calcul stats.fm |
| Premier/dernier stream | historique stocké | timestamps | minimum et maximum | Calcul stats.fm |
| Entités distinctes | historique + catalogue | URI et relations catalogue | déduplication et cardinalité | Calcul stats.fm |
| Séries jour/semaine/mois | historique stocké | timestamp et `ms_played` | regroupement temporel | Calcul stats.fm |
| Horloge d’écoute | historique stocké | heure et durée | agrégation sur 24 heures | Calcul stats.fm |
| Évolution entre périodes | agrégats de deux périodes | volumes et durées | variation absolue et pourcentage | Calcul stats.fm |
| Métadonnées piste | `GET /tracks/{id}` | titre, artistes, album, durée, explicite, ISRC, liens | affichage, cache et rapprochement | Spotify direct |
| Métadonnées album | `/albums/{id}` et `/tracks` | type, date, images, artistes, droits, tracklist | affichage et agrégats personnels | Spotify direct |
| Métadonnées artiste | `GET /artists/{id}` | nom, image, genres | affichage et agrégats | Spotify, selon accès |
| Caractéristiques audio | Audio Features | énergie, valence, tempo, dansabilité, etc. | jauges et moyennes pondérées | Accès restreint/déprécié selon régime |
| Analyse structurelle | Audio Analysis | sections, segments, beats, pitches, timbre | peu exploitée dans l’UI observée | Accès restreint |
| Playlists | endpoints Playlist/Items | propriétaire, visibilité, items, dates | statistiques de playlist | Spotify + calcul |
| Top listeners | base communautaire stats.fm | statistiques de plusieurs utilisateurs | classement communautaire | Propriétaire |
| Records | historique des rangs | rangs et périodes | badges or/argent/bronze | Propriétaire |
| Soulmates | profils et historiques | goûts et écoutes communs | score de similarité | Propriétaire |
| Charts globaux | événements de la communauté | données agrégées | classement global | Propriétaire |

## 4. Données absentes de la Spotify Web API

| Information souhaitée | Web API ? | Alternative |
| --- | --- | --- |
| Historique complet depuis la création du compte | Non | Extended Streaming History |
| Durée réellement écoutée par lecture | Non | `ms_played` de l’export |
| Streams personnels lifetime exacts | Non | comptage local des événements |
| Temps exact par piste, artiste ou album | Non | somme locale de `ms_played` |
| Raison de début ou fin d’une écoute | Non | `reason_start` / `reason_end` de l’export |
| Skip détaillé | Non dans `recently-played` | `skipped` + règles locales |
| Historique hors ligne | Non | `offline` / `offline_timestamp` |
| Historique des sessions privées | Non | `incognito_mode` |
| Appareil historique par écoute | Non | `platform` et données de l’export |
| Top albums ou genres personnel natif | Non | agrégation locale |
| Rang parmi les auditeurs | Non | base communautaire stats.fm |
| Compatibilité entre utilisateurs | Non | algorithme propriétaire |

`recently-played` est une fenêtre récente limitée à 50 éléments par requête et ne fournit pas `ms_played`. Il ne suffit pas pour reconstituer une vie entière d’écoute.

## 5. Données déjà présentes mais encore sous-exploitées

| Donnée | Extension possible | Valeur produit | Faisabilité |
| --- | --- | --- | --- |
| `release_date` | décennies préférées, âge moyen des écoutes, nouveautés vs catalogue ancien | Élevée | Élevée |
| `explicit` | ratio explicite/non explicite par période | Moyenne | Élevée |
| `duration_ms` | distribution des durées et taux de complétion avec `ms_played` | Élevée | Élevée avec export |
| `disc_number` / `track_number` | préférence pour singles, intros, fins d’albums | Moyenne | Élevée |
| ISRC / UPC / EAN | déduplication des remasters et rééditions | Très élevée pour la qualité | Élevée |
| `album_type` | part singles, albums, compilations | Moyenne | Élevée |
| restrictions et marchés | disponibilité géographique et contenus retirés | Faible à moyenne | Moyenne |
| contexte de lecture | part album, playlist, artiste ou contexte inconnu | Élevée | Moyenne, historique incomplet |
| type d’appareil | mobile, ordinateur, enceinte ou voiture | Élevée | Moyenne, collecte continue nécessaire |
| bibliothèque sauvegardée | ancienneté, favoris oubliés, conversion écoute → like | Très élevée | Élevée |
| artistes suivis | suivis vs réellement écoutés, fidélité | Élevée | Élevée selon accès |
| playlists | fraîcheur, doublons, contributeurs, évolution | Très élevée | Élevée selon accès |

## 6. Familles encore largement inexploitées

### 6.1 Bibliothèque personnelle

- date moyenne d’ajout des favoris ;
- favoris jamais ou rarement réécoutés ;
- délai entre première écoute et sauvegarde ;
- conversion « découvert → écouté → sauvegardé » ;
- suppressions et réajouts si des snapshots consentis sont conservés.

### 6.2 Playlists

- évolution et taux de renouvellement ;
- ancienneté moyenne des titres ;
- répartition par contributeur ;
- doublons, artistes dominants et diversité ;
- part des titres réellement écoutés parmi les titres ajoutés.

### 6.3 Contextes, sessions et appareils

Le lecteur fournit appareil actif, contexte, progression, repeat et shuffle. Ces données sont instantanées : elles ne deviennent historiques que si NovaSound les collecte régulièrement avec consentement.

### 6.4 Podcasts et livres audio

Spotify fournit les métadonnées des shows, épisodes, livres audio et chapitres. NovaSound pourrait afficher temps musique vs parole, progression, top shows/auteurs/narrateurs, taux d’achèvement et sessions longues. L’historique récent Web API ne prend actuellement pas en charge les épisodes ; l’export personnel reste nécessaire pour l’historique.

### 6.5 Structure musicale avancée

Audio Analysis permettrait une timeline structurelle, la densité rythmique, la variabilité du tempo, une carte harmonique et une courbe de dynamique. Ce potentiel est différenciant mais risqué pour une nouvelle application à cause des restrictions d’accès.

## 7. Ce qui n’est pas un manque prioritaire

| Fonction Spotify | Pourquoi elle est secondaire pour NovaSound |
| --- | --- |
| Lecture, pause, volume, seek | transforme le produit en télécommande plutôt qu’en outil d’analyse |
| Transfert entre appareils | utile pour un lecteur, faible valeur statistique |
| Gestion de la file d’attente | fonctionnalité d’écoute, pas insight historique |
| Recherche catalogue | utile à la navigation, ne crée pas directement une métrique |
| Création/modification de playlists | action dérivée intéressante, mais secondaire |
| Catégories éditoriales et nouveautés | découverte utile, moins différenciante que les données personnelles |

Ne pas consommer tous les endpoints n’est donc pas nécessairement une lacune produit.

## 8. Restrictions Spotify en 2026

| Situation | Conséquence |
| --- | --- |
| Nouvelle application en Development Mode | endpoints et utilisateurs autorisés limités, quotas contraints |
| Application historique / Extended Quota | périmètre potentiellement plus large et limite supérieure |
| Endpoint déprécié ou restreint | ne pas en faire une dépendance critique sans repli |

Évolutions à intégrer :

- février 2026 : retrait ou modification d’endpoints et champs en Development Mode ;
- mars 2026 : maintien des `external_ids` pour albums et pistes ;
- mai 2026 : ajout de `account_id`, identifiant public, stable et pseudonyme ;
- juillet 2026 : quotas Development Mode comptés par compte développeur, non par Client ID.

## 9. Améliorations recommandées par composant

### 9.1 Accueil / tableau de bord

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| KPI principaux | Les totaux seuls expliquent mal le comportement | Ajouter moyenne et médiane par écoute, variation par rapport à la période précédente et dates exactes de couverture | historique, `ms_played`, timestamps | P0 |
| Tendances | Une hausse ou baisse peut être ambiguë | Afficher valeur actuelle, valeur précédente, écart absolu, pourcentage et période de référence | agrégats de deux périodes | P0 |
| Lecture récente | Manque de contexte analytique | Ajouter durée réellement écoutée, taux de complétion, contexte et origine de la donnée | export + `duration_ms` + contexte API | P1 |
| Recommandations | La raison de la recommandation est peu explicite | Ajouter « Pourquoi cette suggestion ? » avec artistes, genres ou habitudes contributrices | historique + catalogue | P1 |
| Capsule temporelle | Le souvenir est isolé | Comparer l’écoute passée à aujourd’hui et permettre d’ouvrir la période complète | historique multi-années | P2 |

### 9.2 Tops et classements

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Top titres | Rang difficile à interpréter seul | Afficher minutes, streams, part du temps total, durée moyenne et taux de complétion | événements + catalogue | P0 |
| Top artistes | Les collaborations peuvent gonfler les totaux | Signaler la multi-attribution et proposer une vue artiste principal uniquement | crédits artistes + historique | P1 |
| Top albums | Rééditions et remasters fragmentent le classement | Regrouper par édition canonique via ISRC/UPC, avec option « éditions séparées » | IDs externes + catalogue | P1 |
| Top genres | Les genres se chevauchent et ne totalisent pas forcément 100 % | Afficher le dénominateur, les artistes contributeurs et une note sur la multi-attribution | genres artistes + agrégats | P0 |
| Filtres temporels | Les périodes glissantes et calendaires sont confondues | Afficher les dates exactes, le fuseau et le type de période | paramètres de filtre | P0 |
| Pagination | Un Top limité peut sembler exhaustif | Montrer le nombre affiché, le total disponible et un chargement progressif | résultats paginés | P1 |

### 9.3 Historique des écoutes

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Timeline | Navigation difficile sur un historique long | Ajouter recherche, filtres artiste/album/titre/date, regroupement quotidien et pagination | historique importé | P0 |
| Détail d’un événement | La lecture complète ou partielle n’est pas évidente | Afficher `ms_played`, durée catalogue, pourcentage complété, skip et raison de fin | export + catalogue | P0 |
| Provenance | Import et synchronisation peuvent être confondus | Ajouter badge « import Spotify », « synchronisation API » ou « autre source » | métadonnées d’ingestion | P0 |
| Qualité | Les doublons et lignes rejetées sont invisibles | Fournir un rapport de doublons, événements exclus, URI non résolues et corrections | pipeline d’import | P0 |
| Export | L’utilisateur ne peut pas toujours réutiliser ses agrégats | Ajouter export CSV/JSON filtré des données et métriques calculées | base NovaSound | P1 |

### 9.4 Page Statistiques

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Série temporelle | Les jours sans écoute peuvent disparaître | Afficher des intervalles complets avec zéros, choix jour/semaine/mois et comparaison superposée | historique agrégé | P0 |
| Moyennes | Une moyenne seule masque la dispersion | Ajouter médiane, quartiles, P95 et nombre de jours actifs | événements détaillés | P1 |
| Diversité | Le nombre d’entités distinctes reste rudimentaire | Ajouter taux de nouveauté, concentration Top 1/5/10 et entropie | historique + catalogue | P1 |
| Répartition horaire | L’affectation des écoutes traversant deux heures est inconnue | Documenter la règle ou répartir `ms_played` entre les créneaux concernés | timestamps + durées | P1 |
| Genres | Les catégories ne sont pas exclusives | Proposer une vue par association et une vue normalisée, avec explication | genres + pondération | P1 |
| Profil audio | Une moyenne unique peut être trompeuse | Afficher distributions et préciser la pondération par titre, stream ou temps | Audio Features + historique | P2 |
| Couverture | « Lifetime » peut cacher des trous d’import | Ajouter première/dernière date, jours manquants et score de complétude | métadonnées d’import | P0 |

### 9.5 Page Piste

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| KPI personnels | Streams et minutes ne décrivent pas la qualité d’écoute | Ajouter durée moyenne, complétion, skips, premières/dernières écoutes et tendance | export + `duration_ms` | P0 |
| Audio Features | Certaines échelles sont opaques | Afficher valeur, unité, définition et disponibilité ; garder loudness en dB | Audio Features | P1 |
| Radar audio | Les surfaces visuelles peuvent induire en erreur | Conserver le radar comme résumé, mais ajouter le tableau numérique adjacent | Audio Features | P2 |
| Analyse audio | La structure de la piste est peu exploitée | Ajouter sections, changements de tempo, dynamique et densité rythmique si l’accès le permet | Audio Analysis | P2 |
| Versions | Remasters et éditions peuvent diviser les écoutes | Afficher l’édition active et proposer un regroupement par ISRC | ISRC + relations album | P1 |
| Donnée absente | Une valeur manquante peut être interprétée comme zéro | Afficher « non disponible », la cause probable et la source attendue | état de disponibilité | P0 |

### 9.6 Pages Artiste et Album

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| KPI artiste | Les collaborations sont difficiles à interpréter | Ajouter vues « artiste principal » et « toutes collaborations » | crédits artistes | P1 |
| Discographie artiste | L’ordre des albums n’est pas toujours justifié | Afficher date, type, nombre de pistes et clé de tri | catalogue album | P1 |
| Artistes similaires | La méthode et le score ne sont pas visibles | Indiquer la source, la date de calcul et un score de proximité lorsqu’il existe | endpoint ou modèle NovaSound | P2 |
| KPI album | Les différentes éditions fragmentent les données | Proposer regroupement canonique et sélection de l’édition | UPC/ISRC + catalogue | P1 |
| Tracklist album | Aucune contribution personnelle piste par piste | Ajouter streams, minutes, complétion et part du temps de l’album | historique + tracklist | P1 |
| Couverture d’album | Le niveau d’exploration n’est pas mesuré | Ajouter nombre et part des pistes écoutées, ainsi que l’ordre d’écoute | tracklist + historique | P1 |

### 9.7 Profil, social et leaderboards

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Profil | Les totaux ne signalent pas la qualité de couverture | Ajouter période couverte, source des données et score de complétude | imports + synchronisation | P0 |
| Comparaison | Un pourcentage unique cache la méthode | Décomposer le score par artistes, titres, genres, période et taille de l’intersection | historiques des profils consentants | P1 |
| Soulmates | Le score paraît absolu | Afficher dimensions, période, confiance et sensibilité aux données manquantes | algorithme propriétaire | P1 |
| Leaderboards | La population de référence est inconnue | Afficher période, nombre d’utilisateurs éligibles, seuil minimal et couverture | base communautaire | P0 |
| Records | Les badges manquent de contexte | Dater, historiser et relier chaque record au classement correspondant | historique des rangs | P2 |
| Confidentialité | L’effet des réglages reste abstrait | Ajouter aperçu du profil public et explication de l’impact sur comparaisons/classements | préférences de visibilité | P0 |

### 9.8 Import et synchronisation

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Assistant d’import | Le format et les étapes peuvent être confus | Détecter automatiquement les fichiers, expliquer la couverture et valider avant ingestion | archive Spotify | P0 |
| Progression | Un pourcentage ne suffit pas | Afficher étapes : lecture, validation, résolution catalogue, déduplication et agrégation | état du pipeline | P0 |
| Résultat | Les données rejetées sont difficiles à diagnostiquer | Générer un bilan : importées, fusionnées, doublons, exclues, non résolues et erreurs | journal d’import | P0 |
| Synchronisation API | La fenêtre récente peut créer des trous | Afficher dernière synchronisation, prochain passage et période non couverte | tâches de synchronisation | P0 |
| Reprise sur erreur | Un échec peut forcer un nouvel import | Ajouter reprise idempotente et conservation du diagnostic | identifiants de lots | P0 |

### 9.9 Bibliothèque et playlists

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Favoris | Peu exploités dans stats.fm | Ajouter ancienneté, favoris oubliés et taux écoute → sauvegarde | Saved Tracks/Albums + historique | P1 |
| Playlists | Vue principalement descriptive | Ajouter diversité, fraîcheur, doublons et taux de titres réellement écoutés | Playlist Items + historique | P1 |
| Évolution | Les changements de playlist sont perdus | Conserver des snapshots consentis et calculer ajouts/suppressions | `snapshot_id`, `added_at` | P2 |
| Collaboration | Contribution de chaque membre peu visible | Afficher titres ajoutés et diversité par contributeur | `added_by` | P2 |

### 9.10 Partage et récapitulatifs

| Composant | Limite actuelle | Amélioration à apporter | Données nécessaires | Priorité |
| --- | --- | --- | --- | ---: |
| Carte partageable | La métrique peut perdre son contexte | Toujours inclure période, unité, date de génération et source | agrégats + métadonnées | P0 |
| Weekly/Monthly Recap | Le contenu est surtout décoratif | Ajouter comparaison à la période précédente et fait marquant vérifiable | agrégats comparatifs | P1 |
| Personnalisation | Les thèmes portent surtout sur la couleur | Permettre de choisir les métriques visibles sans exposer les données privées | préférences utilisateur | P1 |
| Accessibilité | Le résultat peut dépendre de la couleur | Ajouter texte alternatif, contraste suffisant et libellés explicites | rendu de carte | P1 |

## 10. Backlog recommandé pour NovaSound

| Priorité | Fonctionnalité | Sources | Pourquoi |
| ---: | --- | --- | --- |
| P0 | Import Extended Streaming History | export Spotify | indispensable aux métriques exactes/lifetime |
| P0 | Déduplication par timestamp, URI/ISRC et durée | export + catalogue | évite le double comptage |
| P0 | Indicateur de couverture et trous de données | plages de dates | évite un faux « lifetime » |
| P1 | Complétion et skips | `ms_played`, durée, `skipped`, raisons | insight fort absent de la Web API seule |
| P1 | Analyse de bibliothèque | contenus sauvegardés + historique | forte valeur personnelle |
| P1 | Analyse de playlists | items, dates, auteurs, snapshot + historique | différenciation et rétention |
| P1 | Décennies, explicite et formats | catalogue | simple et fiable |
| P1 | Contextes d’écoute | contexte récent + export | explique comment l’utilisateur écoute |
| P2 | Podcasts et livres audio | export + objets éditoriaux | étend le périmètre |
| P2 | Appareils et sessions | player + export | collecte continue consentie nécessaire |
| P2 | Structure audio avancée | Audio Analysis | différenciant, accès incertain |
| P3 | Contrôle du lecteur | Player | hors cœur analytique |

## 11. Conclusion

stats.fm exploite déjà **la majorité des données pertinentes pour les statistiques musicales**, mais pas toute la Spotify Web API.

Son avantage vient de trois couches :

1. **catalogue Spotify** pour identifier pistes, albums, artistes, images, genres et attributs ;
2. **historique personnel exporté** pour les événements complets et la durée réellement écoutée ;
3. **moteur analytique propriétaire** pour agréger, comparer, classer et socialiser.

La meilleure marge d’amélioration pour NovaSound n’est pas de consommer tous les endpoints, mais de mieux exploiter la bibliothèque, les playlists, les contextes, la qualité de l’historique, la complétion, les skips et les métadonnées temporelles. Podcasts, livres audio et analyse structurelle sont des extensions de second niveau.

## Sources officielles

- [Spotify Web API](https://developer.spotify.com/documentation/web-api)
- [Get Recently Played Tracks](https://developer.spotify.com/documentation/web-api/reference/get-recently-played)
- [Get User's Top Items](https://developer.spotify.com/documentation/web-api/reference/get-users-top-artists-and-tracks)
- [Get User's Saved Tracks](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-tracks)
- [Quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes)
- [Changelog février 2026](https://developer.spotify.com/documentation/web-api/references/changes/february-2026)
- [Changelog mars 2026](https://developer.spotify.com/documentation/web-api/references/changes/march-2026)
- [Changelog mai 2026](https://developer.spotify.com/documentation/web-api/references/changes/may-2026)
- [Changelog juillet 2026](https://developer.spotify.com/documentation/web-api/references/changes/july-2026)
- [Spotify — Understanding your data](https://support.spotify.com/us/article/understanding-your-data/)
