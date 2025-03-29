# Architecture Decision Record (ADR) : Utilisation de Ratatui pour l'Interface Utilisateur

## Submitters

- Groupe Efrei

## Context

### Justification de l'ADR

L'interface utilisateur (UI) est un élément crucial de notre application, car elle permet aux utilisateurs d'interagir avec le système de manière intuitive et efficace. Le choix de la bibliothèque pour construire cette interface est donc une décision architecturale significative. Nous avons opté pour Ratatui pour plusieurs raisons, notamment sa simplicité, sa flexibilité et sa compatibilité avec Rust.

### Approche de Conception

Ratatui est une bibliothèque Rust pour créer des interfaces utilisateur en mode texte. Elle est conçue pour être légère et performante, ce qui en fait un choix idéal pour des applications nécessitant une interface utilisateur réactive et efficace. Ratatui permet de créer des interfaces utilisateur en utilisant des widgets et des layouts, ce qui facilite la gestion de la disposition et de l'interactivité.

## Proposed Design

### Services/Modules Impactés

- **UI Module** : Le module UI sera modifié pour intégrer Ratatui.
- **Main Module** : Le point d'entrée principal de l'application sera modifié pour initialiser et gérer l'interface utilisateur.

### Nouveaux Services/Modules

- **Ratatui Integration Module** : Un nouveau module sera créé pour encapsuler l'intégration de Ratatui et fournir des abstractions pour les widgets et les layouts.

### Impact sur le Modèle et les DTO

- **Modèle** : Aucune modification majeure n'est nécessaire.
- **DTO** : Aucune modification majeure n'est nécessaire.

### Impact sur les API

- **API** : Aucune modification majeure n'est nécessaire.

### Impact sur la Configuration Générale

- **Configuration** : Ajout d'une section de configuration pour Ratatui, incluant des paramètres pour les widgets et les layouts.

### Impact DevOps

- **CI/CD** : Ajout de tests unitaires et d'intégration pour les composants UI.
- **Déploiement** : Assurez-vous que les dépendances de Ratatui sont incluses dans le package de déploiement.

## Considerations

### Alternatives

- **Curses** : Une autre bibliothèque Rust pour les interfaces utilisateur en mode texte, mais moins flexible et moins performante que Ratatui.
- **TUI** : Une bibliothèque Rust pour les interfaces utilisateur en mode texte, mais moins mature et moins bien documentée que Ratatui.

### Concerns

- **Performance** : Ratatui est conçu pour être performant, mais il est important de tester et de profiler l'application pour s'assurer qu'elle répond aux exigences de performance.
- **Complexité** : L'intégration de Ratatui peut ajouter une certaine complexité au code, mais cela est compensé par la flexibilité et la puissance de la bibliothèque.

### Résolution des Concerns

- **Performance** : Des tests de performance seront réalisés pour s'assurer que l'application répond aux exigences.
- **Complexité** : Des abstractions seront créées pour simplifier l'utilisation de Ratatui et réduire la complexité du code.

## Decision

### Important Implementation Detail

- **Initialisation de Ratatui** : L'initialisation de Ratatui sera gérée dans le module principal de l'application.
- **Gestion des Widgets** : Les widgets seront gérés de manière abstraite pour faciliter les modifications futures.

### Caveats

- **Compatibilité** : Assurez-vous que toutes les dépendances de Ratatui sont compatibles avec les autres bibliothèques utilisées dans le projet.

### Future Considerations

- **Mise à Jour de Ratatui** : Suivre les mises à jour de Ratatui et les intégrer dans le projet pour bénéficier des nouvelles fonctionnalités et des corrections de bugs.
- **Extensibilité** : Ajouter des fonctionnalités supplémentaires pour les widgets et les layouts en fonction des besoins futurs.

### Remaining or Deferred Design Issues

- **Accessibilité** : Assurez-vous que l'interface utilisateur est accessible à tous les utilisateurs, y compris ceux utilisant des technologies d'assistance.

## References

- [Ratatui Documentation](https://docs.rs/ratatui/latest/ratatui/) - Documentation officielle de Ratatui.
- [Rust Programming Language](https://www.rust-lang.org/) - Documentation officielle de Rust.