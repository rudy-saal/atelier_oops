# Atelier - Configuration de projet

> ⚠️ Cet atelier vous requiert d'être autonome. Vous devrez naviguer parmi de la documentation, faire des recherches exhaustive, installer plusieurs logiciels, etc.

Cet atelier a pour but de vous montrer comment différents projets logiciels sont configurés en fonction des langages et des outils utilisés. Votre mission est de modifier les fichiers de configuration afin que le code et les tests de chaque projet puissent s'exécuter.

Pour ce faire, vous devez :

1. Définir les [métadonnées](#métadonnées) requises pour chaque projet;
2. Installer les logiciels nécéssaire à l'interprétation ou la compilation de chaque projet, en fonction du language utilisé;
3. Trouver, définir et installer les dépendances (librairies) d'exécution manquantes pour chaque projet en suivant le compilateur (les dépendances de développement vous sont fournises).

Les projets à corriger sont :

- [python](./python) : Un projet dans le langage [Python](https://www.python.org/) monté avec l'outil [poetry](https://python-poetry.org);
- [rust](./rust) : Un projet dans le langage [Rust](https://www.rust-lang.org/) monté avec l'outil standard [cargo](https://doc.rust-lang.org/cargo/);
- [typescript](./typescript) : Un projet dans le langage [Typescript](https://www.typescriptlang.org/) monté avec l'outil standard [npm](https://docs.npmjs.com/).

## Informations additionnels

- Les fichiers de configurations sont déjà existants.
- Les sections de configuration manquantes sont indiquées par des `TODO`.
- Lisez bien les sections pertinentes des documentations de chaque outil et language afin de trouver comment structurer les informations manquantes.
- Le code nécéssaire à l'exécution vous est déjà fourni, donc pas besoin d'apprendre ou de comprendre les différents langages.
- Il vous faudra installer les logiciels permettant l'exécution (à la bonne version!) des projets.
  - Vous êtes libre de soit installer les logiciels directement, soit installer des environnement virtuels ou encore mettre en place des conteneurs de style Docker.
- La majorité des commandes pour *compiler* , *exécuter* et *tester* le code de chaque projet vous sont déjà fournis, soit dans le fichier de configuration, soit par l'entremise de scripts `bash`. Pour les commandes manquantes, ce sont à vous de les trouver.
  - Vous pouvez utiliser l'outil Git Bash (installable avec Git) si vous êtes sur Windows, ou simplement copier-coller leur contenu pour exécuter dans Powershell.

## Critères de succès

1. Le fichier de configuration est valide et contient les bonnes métadonnées
   - nom: ping
   - description: A nice little PING tool.
   - auteur: Nice people Inc.
   - version: 0.1.0
   - license: MIT 
2. Aucune erreur de compilation
3. Aucune erreur de test
4. L'exécution du programme (avec les arguments requis et un URL fonctionnel) affiche un texte du genre:

```text
SUCCESS RATE : 100%
SUCCESS RATE : 100%
SUCCESS RATE : 100%
SUCCESS RATE : 100%
SUCCESS RATE : 100%
SUCCESS RATE : 100%
Successfuly pinged 100% of the requests over 5 tries.
```

## Logiciels intéressants

- asdf
- nvm
- pyenv
- vscode
