# Rapport 1 : Audit UI, metriques et lecture statistique de stats.fm

> Analyse refaite le 1er septembre 2026 avec Playwright sur les pages publiques et
> authentifiees du site Web stats.fm. Ce rapport decrit non seulement ce qui est
> affiche, mais aussi ce que chaque mesure permet ou ne permet pas de conclure.

## Methodologie et niveau de preuve

- **Observe public** : verifie sans authentification dans l'interface Web.
- **Observe authentifie** : verifie apres connexion au compte du profil `stenguyz`.
- **Observe dans les donnees de page** : verifie dans la reponse publique utilisee par l'interface, sans reproduire de cookie, jeton ou secret.
- **Documente** : decrit dans un support officiel, sans nouvelle verification dans cette passe.
- **Documente mobile** : visible dans les supports officiels, mais non retrouve dans le site Web explore.
- **Proprietaire** : calcule ou gere par stats.fm, sans equivalent direct dans Spotify Web API.
- **Calcul confirme** : les valeurs detaillees se reconcilient avec le total affiche.
- **Calcul probable** : les observations concordent, mais la formule n'est pas documentee ou tous les cas limites ne sont pas connus.
- Les nombres personnels sont des instantanes. Ils changent avec les nouvelles ecoutes, imports, corrections de catalogue et regles de traitement.

## Definitions indispensables

### Evenement, stream et temps ecoute

- Le grain de base est un **evenement d'ecoute** : un contenu, un timestamp et une duree reellement ecoutee en millisecondes.
- Dans ce rapport, un **stream comptabilise** est un evenement retenu dans les statistiques stats.fm. Le support indique que les ecoutes de moins de 30 secondes, les pistes marquees comme skippees, les fichiers locaux et les podcasts sont exclus des statistiques actuelles.
- La cle exacte de deduplication entre import et synchronisation, le traitement des valeurs nulles et les autres controles de qualite ne sont pas exposes. Le terme `stream` ne signifie donc pas automatiquement une lecture complete.
- `minutes streamed` est une somme de `playedMs`, pas une somme des durees catalogue des pistes.
- `different tracks`, `different artists` et `different albums` sont des cardinalites selon la resolution d'entites de stats.fm. Une reedition, un remaster ou plusieurs identifiants externes peuvent modifier ces comptes.

Les noms varient selon la source : l'export Spotify nomme la duree evenementielle
`ms_played`, les lignes de top stats.fm utilisent `playedMs` et l'agregat de profil
utilise `durationMs`. Ils sont tous exprimes en millisecondes, mais le rapport conserve
leur nom source afin de ne pas masquer les filtres ou transformations intermediaires.

### Lifetime et couverture

`lifetime` signifie toute la periode couverte par les evenements actuellement stockes
par stats.fm. Cela ne garantit ni une couverture depuis la creation du compte Spotify,
ni l'absence de trou entre deux synchronisations. Les valeurs sont des totaux sur la
couverture disponible, pas necessairement des totaux de vie complets du compte.

### Rang et multi-attribution

- Les Top tracks, Top artists, Top albums et Top genres testes sont tries par `playedMs` decroissant. Les streams sont affiches comme seconde mesure, mais ne commandent pas le rang.
- La regle de departage lorsque deux entites ont le meme `playedMs` n'a pas ete determinee.
- Une piste avec plusieurs artistes credite l'evenement a plusieurs artistes dans les tops. Un artiste present dans plusieurs genres credite aussi plusieurs genres. Les sommes par artiste ou par genre peuvent donc depasser le total de streams du profil.
- Les pourcentages par artiste ou genre ne forment pas necessairement une partition de 100 %. Ils mesurent une association, pas des categories mutuellement exclusives.

## Synthese ligne par ligne

| Page / vue | Ce qui est reellement affiche ou mesure | Lecture statistique | Statut | Avis et amelioration |
| --- | --- | --- | --- | --- |
| Landing page | Exemples de top tracks, artistes, albums, historique et graphiques | Demonstration du produit, pas un echantillon analysable | Observe public | Bien pour comprendre le produit, trop vague pour evaluer les metriques; publier un catalogue versionne des graphiques |
| Landing page | Compteurs d'utilisateurs, Plus, streams, pistes, artistes et albums connus | Taille cumulee de la base stats.fm, sans periode ni population eligible detaillee | Observe public, proprietaire | Interessant pour l'echelle; ajouter date de mise a jour, definition et evolution temporelle |
| Accueil authentifie | Avatar et menu `My page`, `Settings`, `Log out` | Navigation uniquement, aucune mesure | Observe authentifie | Correct; aucun enjeu statistique |
| Profil | Avatar, badge Plus, nom, bio, amis et connexions | Identite du profil et taille brute de son graphe social | Observe authentifie | Utile comme contexte; ne pas confondre nombre d'amis et representativite de la comparaison |
| Profil, KPI | Streams et somme du temps ecoute | Deux mesures de volume : frequence des evenements et exposition temporelle | Observe authentifie, calcul confirme | Tres utile; ajouter duree moyenne, mediane, intervalle interquartile et couverture de dates |
| Profil, KPI | Heures ecoutees | Conversion arrondie du temps total | Observe authentifie, calcul confirme | Lisible, mais redondant avec les minutes; afficher la regle d'arrondi |
| Profil, KPI | Pistes, artistes et albums distincts | Diversite brute des entites rencontrees pendant la periode | Observe authentifie | Utile mais insuffisant; ajouter taux de nouveaute, entropie et cle de deduplication |
| Profil, periode | `today`, `this week`, `4 weeks`, `6 months`, annee, `lifetime` | Filtre commun applique aux KPI, tops, genres et horloges | Observe authentifie | Bonne coherence; afficher les dates exactes, le fuseau et le caractere glissant ou calendaire |
| Profil, genres | Tags ordonnes avec `streams`, `playedMs` et artistes contributeurs dans les donnees de page | Intensite d'ecoute associee a chaque genre; categories chevauchantes | Observe authentifie | Le simple tag cache presque toute la statistique; afficher minutes, part du total, artistes contributeurs et avertissement de multi-attribution |
| Profil mobile, genres | Barres horizontales de top genres | Comparaison de poids entre genres, formule non visible dans l'audit Web | Documente mobile | Meilleur que les tags si l'unite et le denominateur sont affiches |
| Profil, Top tracks | Rang, titre, artistes, minutes et streams | Classement des pistes par temps ecoute total; les streams indiquent la repetition | Observe authentifie | Bon duo de mesures; ajouter duree moyenne par stream, part du temps total et taux de completion |
| Profil, Top artists | Rang, artiste, minutes et streams | Concentration du temps sur les artistes; attribution multiple possible | Observe authentifie | Tres utile; ajouter part cumulee Top 1/5/10 et signaler les collaborations comptees plusieurs fois |
| Profil, Top albums | Rang, album, minutes et streams | Concentration par edition d'album telle que resolue par stats.fm | Observe authentifie | Utile, mais les reeditions fragmentent l'analyse; proposer regroupement canonique et vue par sortie |
| Profil, grilles | Routes `/tracks`, `/artists`, `/albums` et mode grille | Autre presentation des memes tops; l'audit a vu 250 pistes, 250 artistes et 172 albums retournes sur quatre semaines | Observe authentifie | La profondeur est bonne, mais elle n'est pas exhaustive face aux 900 pistes et 655 albums distincts; indiquer limite et pagination |
| Profil, partage | Action `Share` sur les tops | Diffusion d'un resultat, aucune statistique supplementaire | Observe authentifie | Correct; inclure periode, unite et date dans l'image partagee |
| Profil, horloge streams | 24 valeurs horaires dont la somme egale le nombre de streams | Frequence d'evenements selon l'heure de debut dans le fuseau demande | Observe authentifie, calcul confirme | Tres utile pour le rythme journalier; afficher valeurs, fuseau et pourcentage au survol |
| Profil, horloge temps | 24 sommes de `durationMs` dont le total egale le temps du profil | Volume ecoute par heure d'affectation | Observe authentifie, calcul confirme | Complementaire; documenter si une ecoute traversant deux heures reste entierement dans l'heure de debut |
| Profil, Recent streams | Evenements groupes par date avec piste, artiste, album et temps relatif | Sequence recente, utile pour inspecter la recence mais pas pour resumer une tendance | Observe authentifie | Bon controle qualite; ajouter duree ecoutee et contexte de lecture directement sur le profil |
| Historique | Timeline progressive avec date, piste, duree ecoutee et recence | Journal evenementiel permettant de verifier les agregats | Observe authentifie | Excellente base; manque filtres, export, recherche, regroupement et indicateur d'import/synchronisation |
| Artiste, KPI personnels | Streams, minutes, premier et dernier stream quand disponibles | Intensite et anciennete de la relation entre utilisateur et artiste | Observe authentifie | Tres utile; afficher la periode couverte et gerer explicitement les valeurs manquantes, vues comme `-` pendant le test |
| Artiste, catalogue | Followers, score de popularite sur 10 et genres | Portee catalogue globale et etiquettes editoriales, pas comportement personnel | Observe authentifie | Contextualise l'artiste, mais les valeurs observees peuvent etre nulles ou obsoletes; afficher source et date |
| Artiste, Top tracks | Titres catalogue associes a l'artiste | Popularite catalogue, sans metrique visible dans la liste testee | Observe authentifie | Faible analytiquement sans score, marche ni periode; ajouter la metrique qui justifie l'ordre |
| Artiste, Top albums | Albums catalogue associes a l'artiste | Discographie ordonnee par une regle non exposee | Observe authentifie | Meme limite; afficher date, type, popularite et cle de tri |
| Artiste, Related artists | Artistes indiques comme similaires | Proximite catalogue ou d'audience selon une methode externe non exposee | Observe authentifie | Utile pour decouverte, faible comme statistique sans score, marche et methode |
| Artiste, leaderboard global | Rang, profil, minutes et streams | Position d'un auditeur parmi les utilisateurs stats.fm eligibles, triee par minutes | Observe authentifie | Differenciant, mais fortement biaise par l'auto-selection et la couverture d'import; afficher population eligible et periode |
| Artiste, leaderboard amis | Meme classement limite aux amis stats.fm | Comparaison dans un petit sous-groupe social | Observe authentifie | Ludique, non inferentiel; afficher taille du groupe, utilisateurs exclus et periode |
| Artiste, Your streams | Timeline des pistes comportant l'artiste, avec duree ecoutee | Verification evenementielle de l'agregat artiste | Observe authentifie lors du second test | Tres utile; corrige l'audit initial qui ne l'avait pas trouvee; preciser la regle pour les featuring |
| Piste, KPI personnels | Total de streams et de minutes ecoutees | Repetition et exposition cumulee a une piste | Observe authentifie | Bon minimum; ajouter moyenne `minutes/stream`, completion et skips |
| Piste, catalogue | Duree, artiste, albums d'apparition et popularite sur 10 | Contexte catalogue pour interpreter l'exposition | Observe authentifie | Utile; distinguer clairement duree catalogue et duree reellement ecoutee |
| Piste, leaderboard global | Auditeurs classes avec minutes et streams | Intensite comparee sur une meme piste parmi les participants stats.fm | Observe authentifie | Potentiellement interessant; periode, eligibilite et taille de population doivent etre visibles |
| Piste, leaderboard amis | Meme classement sur le sous-groupe d'amis | Comparaison sociale locale | Observe authentifie | Correct comme mecanique sociale, faible valeur statistique sans taille d'echantillon |
| Piste, barres audio | Huit caracteristiques audio, dont loudness | Profil descriptif de la piste, pas du comportement de l'utilisateur | Observe authentifie lorsque disponible | Bonne lecture rapide; loudness en dB ne doit pas etre traite comme un score 0-1 |
| Piste, radar audio | Sept axes normalises, sans loudness | Forme relative du profil acoustique d'une piste | Observe authentifie lorsque disponible | Visuel mais imprecis; ajouter valeurs et echelles, eviter les comparaisons d'aire trompeuses |
| Piste, cartes audio | Loudness, tonalite, mode, signature et BPM | Parametres musicaux numeriques de la piste | Observe authentifie lorsque disponible | Meilleure vue pour une lecture exacte; ajouter definitions et incertitude de l'analyse automatique |
| Piste, audio absent | `No audio features available for this track` | Absence de mesure, pas valeur nulle | Observe authentifie | Bonne gestion; expliquer la cause et ne jamais imputer zero |
| Piste, Recent streams | Evenements de cette piste, dates et durees ecoutees | Distribution des repetitions et lectures partielles | Observe authentifie | Tres utile; ajouter completion, source, skip et export |
| Album, KPI personnels | Streams, minutes, premier et dernier stream si disponibles | Volume d'ecoute cumule sur toutes les pistes rattachees a cette edition | Observe authentifie | Utile; clarifier edition, fusion de versions et valeurs manquantes |
| Album, catalogue | Nombre de pistes, popularite, type et date de sortie | Contexte de taille, format et anciennete de la sortie | Observe authentifie | Bon contexte; afficher precision de date et edition canonique |
| Album, tracklist | Numero, titre et artistes | Composition de l'album, sans mesure personnelle ligne par ligne | Observe authentifie | Correct; ajouter streams, minutes et completion par piste pour comprendre quelles pistes portent l'album |
| Album, leaderboard global | Auditeurs classes par minutes avec streams affiches | Intensite comparee parmi les participants stats.fm | Observe authentifie | Meme biais que l'artiste; ajouter periode, population et seuil d'eligibilite |
| Album, leaderboard amis | Classement limite au graphe d'amis | Comparaison sociale locale | Observe authentifie | Ludique; afficher `n` et couverture |
| Album, Your streams | Timeline des ecoutes de toutes les pistes de l'album | Trace evenementielle de l'agregat album | Observe authentifie | Tres utile; permettre regroupement par piste et taux de couverture de l'album |
| Genre | Artistes associes avec followers et etiquette | Catalogue d'artistes portant ce tag, pas distribution personnelle | Observe authentifie | Trop pauvre; afficher definition du genre, poids personnel, artistes contributeurs et source des followers |
| Recherche | Sections Artists, Albums, Tracks et Users | Resultats catalogue plus index social stats.fm | Observe authentifie | Fonctionnel; afficher pertinence, type de correspondance et source de chaque resultat |
| Amis | Profils, avatars, noms et pronoms | Graphe social explicite | Observe authentifie | Utile pour les comparaisons; aucune conclusion populationnelle ne doit en etre tiree |
| Confidentialite | Visibilite independante du profil, ecoute, tops, genres, streams, stats et leaderboards | Controle de quelles observations entrent dans les vues publiques et sociales | Observe authentifie | Tres bon niveau de controle; ajouter apercu public et explication de l'impact sur les classements |
| Imports | Spotify/Apple Music, `.json`/`.zip`, depot et statut | Acquisition de l'historique evenementiel et suivi de traitement | Observe authentifie | Essentiel; ajouter couverture detectee, doublons, rejets, erreurs et rapport de qualite |
| Connexions | Etat des services externes; streaming indique `Coming soon` sur cette page | Etat d'integration, aucune mesure | Observe authentifie | Information claire mais incoherente avec les donnees deja synchronisees; expliquer les canaux actifs |
| Mobile, serie temporelle | Streams et temps ecoute par intervalle | Tendance, saisonnalite et ruptures d'activite | Documente mobile, non observe Web | Indispensable pour un analyste; priorite Web elevee avec choix du grain et intervalles complets |
| Mobile, moyennes | Moyennes quotidiennes et variation entre periodes | Niveau moyen et evolution relative | Documente mobile, non observe Web | Utile seulement si denominateur, periode de reference et cas zero sont documentes |
| Mobile, resume audio | Moyennes de caracteristiques audio sur un ensemble ecoute | Profil acoustique agrege de la consommation | Documente mobile, non observe Web | Interessant, mais la ponderation par piste, stream ou temps change completement le resultat |
| Mobile, Soulmates | Compatibilite et contenus communs | Similarite entre deux profils selon un algorithme proprietaire | Documente mobile | Ludique; afficher dimensions, periode, taille d'intersection et sensibilite du score |
| Mobile, Records | Medailles liees aux classements | Gamification de positions sociales | Documente mobile | Engageant mais pas analytique; dater et historiser chaque record |
| Charts globaux | Artistes, pistes et albums classes sur les streams stats.fm | Demande agregee de la population stats.fm participante | Documente, proprietaire | Interessant comme panel, non representatif de Spotify; publier couverture, pays, periode et methode |

## Resultats des nouveaux tests quantitatifs

### Plages temporelles et bornes observees

| Libelle UI | Parametre de route | Comportement observe | Interpretation et limite |
| --- | --- | --- | --- |
| `today` | `range=today` | Requete du minuit local au minuit suivant | Jour calendaire dans le fuseau transmis par la page; borne finale observee differente des autres filtres, donc convention d'inclusivite a documenter |
| `this week` | `range=this_week` | Du dimanche 00:00 au samedi 23:59:59 dans le test | Semaine calendaire dimanche-samedi, pas sept jours glissants |
| `4 weeks` | `range=weeks` | Plage nommee par le serveur | Probablement glissante, mais les bornes exactes ne sont pas exposees dans l'URL appelee |
| `6 months` | `range=months` | Plage nommee par le serveur | Probablement glissante, mais definition exacte non confirmee |
| `2026` | `range=current_year` | Du 1er janvier au 31 decembre dans le test | Annee calendaire complete, y compris la partie future qui ne contient evidemment aucun evenement |
| `lifetime` | `range=lifetime` | Tous les evenements stockes | Couverture disponible, pas garantie d'exhaustivite du compte |

La page a transmis `America/Toronto` au calcul des statistiques par date lors du test.
Cela confirme que le fuseau intervient dans l'affectation des heures et jours. Le rapport
ne conclut pas que ce fuseau est fixe : il peut provenir du navigateur, du profil ou
d'un parametre d'application.

### Instantane quatre semaines

Releve final du test local du 1er septembre 2026 :

| Metrique | Valeur | Definition | Ce qu'un analyste peut lire |
| --- | ---: | --- | --- |
| Streams | 1 798 | Nombre d'evenements retenus | Volume de repetitions, pas nombre de pistes completes |
| Minutes | 5 558 affichees | `333 530 135 ms / 60 000`, puis affichage sans decimales | Environ 92,65 heures d'exposition cumulee |
| Heures | 93 | Conversion arrondie du temps en heures | Valeur lisible mais moins precise que les millisecondes |
| Pistes distinctes | 900 | Cardinalite piste selon stats.fm | Environ une piste differente pour deux streams, sans mesurer l'equite de la distribution |
| Artistes distincts | 535 | Cardinalite artiste | Diversite d'artistes, affectee par les collaborations |
| Albums distincts | 655 | Cardinalite album | Diversite d'editions, affectee par singles, compilations et reeditions |
| Duree moyenne par stream | 185,5 s | `durationMs / count` | Une ecoute comptabilisee dure en moyenne 3 min 05 s |
| Mediane par stream | 180,0 s | 50e percentile de `playedMs` | La moitie des streams dure au plus 3 minutes et l'autre au moins 3 minutes |
| Intervalle interquartile | 160,2 a 211,0 s | 25e et 75e percentiles | La moitie centrale des ecoutes dure environ 2 min 40 s a 3 min 31 s |
| P95 / P99 | 284,0 / 360,0 s | Percentiles superieurs | Environ 5 % se situent dans la queue au-dessus du P95 et environ 1 % au-dessus du P99, sous reserve de la methode d'interpolation et des egalites |

Les percentiles, minimum, maximum et moyenne sont presents dans la reponse utilisee
par la page, mais ne sont pas affiches dans l'interface Web. C'est une perte importante
pour l'analyse : le total seul ne distingue pas de nombreuses ecoutes courtes de
quelques longues ecoutes.

### Verification du tri des tops

| Top teste | Nombre de lignes retournees | Tri `playedMs` decroissant | Tri streams decroissant | Conclusion |
| --- | ---: | --- | --- | --- |
| Tracks | 250 | Oui sur toutes les lignes | Non | Le rang mesure le temps cumule par piste |
| Artists | 250 | Oui sur toutes les lignes | Non | Le rang mesure le temps associe a l'artiste |
| Albums | 172 | Oui sur toutes les lignes | Non | Le rang mesure le temps cumule par edition d'album |
| Genres | 126 | Oui sur toutes les lignes | Non | Le rang mesure le temps associe aux artistes portant le genre |

Les 250 pistes retournees ne representent que 250 des 900 pistes distinctes de la
periode. Additionner ce tableau ne redonne donc pas le total. Les sommes artistes et
genres peuvent au contraire depasser le total a cause de la multi-attribution.

### Lecture statistique d'un Top artist

La premiere ligne observee sur quatre semaines affichait 1 629 minutes et 488 streams.
Un analyste peut en deduire :

- environ 29,3 % du temps total du profil etait associe a des pistes creditees a cet artiste;
- environ 27,1 % des streams etaient associes a cet artiste;
- le rapport `playedMs / streams` etait proche de 200 secondes par attribution;
- la part en temps superieure a la part en streams suggere des ecoutes associees legerement plus longues que la moyenne globale.

Ce n'est pas une part exclusive : une collaboration peut etre creditee a plusieurs
artistes, donc les parts de tous les artistes ne doivent pas etre additionnees pour
obtenir 100 %.

### Lecture des Listening clocks

Les 24 valeurs de l'horloge streams totalisaient exactement 1 798. Les 24 valeurs de
l'horloge temps totalisaient exactement `333 530 135 ms`, soit le total du profil.
Les heures avec le plus d'evenements etaient 14 h, 10 h et 9 h dans le fuseau demande;
les heures avec le plus de temps etaient 10 h, 9 h et 14 h.

Cela permet de distinguer :

- **frequence horaire** : combien d'ecoutes commencent ou sont attribuees a une heure;
- **volume horaire** : combien de temps ecoute est attribue a cette heure;
- **duree moyenne horaire** : `durationMs / count`, calculable mais non affichee;
- **creux et pics d'habitude** : utiles descriptivement pour ce profil, sans prouver une causalite.

La methode d'affectation d'une ecoute qui traverse deux heures n'est pas documentee.
Le graphique ne doit donc pas etre presente comme une occupation exacte minute par
minute sans verification supplementaire.

## Audio Features : ce que les axes veulent dire

| Axe | Nature de la mesure | Lecture utile | Limite | Avis |
| --- | --- | --- | --- | --- |
| Danceability | Score de compatibilite avec la danse | Compare rythme, regularite et sensation dansante entre pistes | Ce n'est pas une probabilite de danser | Utile en comparaison, peu parlant seul |
| Energy | Score perceptif d'intensite | Distingue pistes calmes et intenses | Ne remplace pas loudness ou tempo | Bon axe de segmentation |
| Loudness | Niveau sonore moyen en dB | Compare la puissance sonore moyenne | Echelle negative et non 0-1 | Doit rester en dB, pas dans une barre normalisee opaque |
| Speechiness | Presence estimee de parole | Distingue musique, rap parle, discours ou podcast | Seuils et erreurs possibles | Utile pour typer le contenu |
| Acousticness | Confiance dans le caractere acoustique | Oppose approximativement acoustique et production electronique | Ne dit pas quels instruments sont presents | Bon pour exploration, pas preuve musicologique |
| Instrumentalness | Probabilite d'absence de voix | Isole les pistes principalement instrumentales | Choeurs et samples peuvent perturber | Utile avec un seuil explicite |
| Liveness | Presence probable d'un public ou contexte live | Distingue studio et enregistrement live | Une ambiance de foule peut tromper le modele | A lire comme indice, pas certitude |
| Valence | Positivite musicale percue | Compare tonalite emotionnelle positive ou sombre | Ne mesure pas l'emotion reelle de l'auditeur | Interessant agrege, avec prudence culturelle |
| Tempo | Battements par minute | Vitesse rythmique estimee | Demi/double tempo possibles | Metrique concrete, utile avec distribution |
| Key / mode | Tonalite et majeur/mineur | Profil harmonique catalogue | Detection automatique imparfaite | Utile pour musiciens, secondaire pour comportement |
| Time signature | Signature rythmique estimee | Structure metrique de la piste | Peu informative sans contexte | Afficher, mais ne pas survaloriser |

Un resume de profil doit preciser sa ponderation. La moyenne des pistes distinctes
repond a « a quoi ressemble le catalogue explore », la moyenne ponderee par streams a
« a quoi ressemblent les lectures », et la moyenne ponderee par `playedMs` a « a quoi
ressemble le temps d'exposition ». Ces trois questions produisent des resultats
differents.

## Leaderboards : interpretation et biais

Les leaderboards testes sont ordonnes par minutes decroissantes et affichent aussi les
streams. Ils indiquent donc une intensite cumulee parmi les comptes stats.fm eligibles.

Ils ne mesurent pas :

- le rang parmi tous les auditeurs Spotify;
- la popularite de l'artiste dans la population generale;
- une comparaison equitable si les utilisateurs n'ont pas la meme couverture d'import;
- une performance sur une periode connue, car aucun libelle temporel n'etait visible sur les pages d'entite testees.

La population est auto-selectionnee : inscription a stats.fm, import, synchronisation,
reglages de confidentialite et qualite des donnees influencent l'eligibilite. Mon avis :
le leaderboard est une excellente fonctionnalite sociale, mais une mauvaise source
d'inference populationnelle tant que `n`, periode, couverture et regles d'eligibilite
ne sont pas affiches.

## Qualite, confidentialite et tracabilite

L'interface de confidentialite permet de regler separement profil, lecture actuelle,
50 derniers titres, tops, genres, streams, statistiques, leaderboards, amis et
connexions. C'est un point fort. Une absence dans une page publique peut donc signifier
« masque » plutot que « inexistant ».

L'import Web accepte Spotify et Apple Music, avec `.json` et `.zip` pour Spotify, puis
affiche le statut de traitement. Il manque cependant un rapport de qualite comprenant :

- premiere et derniere date couvertes;
- jours ou periodes sans evenement;
- evenements lus, retenus, exclus et dupliques;
- URI ou metadonnees non resolues;
- repartition import contre synchronisation;
- version des regles de calcul appliquees.

## Avis global sur les donnees stats.fm

### Bonnes choses

- Le grain evenementiel et `playedMs` sont beaucoup plus riches que le simple ordre algorithmique de Spotify `/me/top`.
- Les totaux, tops, timelines et horloges sont reconcilables sur le test : les sommes horaires correspondent aux KPI.
- La double lecture streams/minutes distingue repetition et temps d'exposition.
- Les pages d'entite relient agregat, catalogue, historique personnel et comparaison sociale.
- Les percentiles et cardinalites existent deja dans les donnees utilisees par la page.
- Les controles de confidentialite sont granulaires.

### Mauvaises choses

- L'interface cache les definitions, formules, bornes exactes et percentiles disponibles.
- Le terme `lifetime` donne une impression d'exhaustivite que la couverture d'import ne garantit pas.
- Les tops ne signalent pas clairement qu'ils sont tries par temps et qu'ils sont tronques.
- Les genres et artistes multi-labels rendent les parts non additives, sans avertissement.
- Popularite et followers peuvent etre nuls, anciens ou dependants d'acces Spotify historiques.
- Les leaderboards ne montrent ni periode, ni population eligible, ni couverture, ce qui invite a une interpretation excessive.
- Les versions, remasters, singles et albums peuvent fragmenter les entites.
- Les vues Web montrent surtout des totaux et classements, peu de distributions ou series temporelles.

### Ameliorations prioritaires

1. Ajouter un dictionnaire de metriques accessible depuis chaque carte : formule, unite, grain, filtre, fuseau et couverture.
2. Afficher les dates exactes de chaque periode et une jauge de completude de l'import.
3. Exposer moyenne, mediane, quartiles, P95, minimum et maximum de `playedMs` deja disponibles.
4. Ajouter series temporelles jour/semaine/mois avec intervalles vides explicites.
5. Montrer parts et concentration Top 1/5/10, tout en expliquant la multi-attribution.
6. Ajouter completion, ecoutes partielles et skips avec denominateurs explicites.
7. Rendre les tops pagines et annoncer `250 sur 900`, plutot que laisser croire a l'exhaustivite.
8. Canonicaliser ou permettre de fusionner remasters, reeditions et identifiants multi-services.
9. Afficher `n`, periode, eligibilite et couverture dans chaque leaderboard.
10. Fournir un rapport de qualite apres chaque import et un export des agregats calcules.

## Limites de l'audit

- L'audit porte sur le Web desktop, pas sur l'execution native Android ou iOS.
- Les donnees du profil ont evolue entre les captures initiales et le second test.
- Les formules internes non exposees restent qualifiees de probables, meme lorsqu'elles correspondent aux exemples.
- Certaines pistes n'ont aucune Audio Feature; une absence n'est pas un score nul.
- Le test n'a pas modifie les reglages, imports ou donnees du compte.

## Sources

- [stats.fm](https://stats.fm/)
- [Profil audite](https://stats.fm/stenguyz)
- [stats.fm Plus](https://stats.fm/plus)
- [Support stats.fm : import](https://support.stats.fm/docs/import/)
- [Support stats.fm : methodes de calcul](https://support.stats.fm/docs/import/faq/calculation-methods)
- [Support stats.fm : synchronisation](https://support.stats.fm/docs/streams/sync)
- [App Store : stats.fm](https://apps.apple.com/app/spotistats-for-spotify/id1526912392)
- [Google Play : stats.fm](https://play.google.com/store/apps/details?id=dev.netlob.spotistats)
