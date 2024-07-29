# Gestion de budget
Ce projet perso est mon support d'apprentissage du langage Rust.

Utilisable en ligne de commande, il créé un fichier `$HOME/.budg/budget.json`. L'interaction se fait en ligne de commande :

## Commandes
- `cargo run list [--id]`  
Permet de lister les dépenses enregistrées. L'option `--id` permet l'affichage de l'identifiant de chaque dépense enregistrée.

- `cargo run delete <id>`
Supprime l'enregistrement dont l'identifiant est `<id>`

- `cargo run add <name> <amount> <day>`
Créé un enregistrement pour la dépense fixe dont le nom sera `<name>`, le montant `<amount>` et le jour du prélèvement `<day>`.

## todo
- définition d'un salaire pour l'affichage d'une jauge représentant les charges fixes et le reste à vivre
- module de migration des données, d'une version à la suivante
- refactoriser les accès fichiers
- création d'un historique par mois
- création d'une valeur d'épargne
- gestion de dépenses ponctuelles

## à explorer
Crates pour la customisation du terminal : 
- [Termion](https://docs.rs/termion/latest/termion/)
- [TUI](https://docs.rs/tui/latest/tui/)

Crates pour une interface graphique : 
- [egui](https://github.com/emilk/egui)
- [druid](https://docs.rs/druid/latest/druid/)