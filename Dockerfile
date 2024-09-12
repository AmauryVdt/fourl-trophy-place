# Étape 1 : Utiliser l'image officielle de Rust
FROM rust:latest AS builder

# Installer Trunk, l'outil pour compiler les projets Yew
RUN cargo install trunk

# Créer le répertoire de travail à l'intérieur du conteneur
WORKDIR /app

# Copier Cargo.toml et Cargo.lock pour installer les dépendances
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances
RUN cargo fetch

# Copier le reste du code source
COPY . .

# Compiler le projet avec Trunk pour générer le WebAssembly
RUN trunk build --release

# Étape 2 : Utiliser une image NGINX pour servir les fichiers WebAssembly et HTML
FROM nginx:alpine AS webserver

# Copier les fichiers générés par Trunk dans le répertoire public de NGINX
COPY --from=builder /app/dist /usr/share/nginx/html

# Exposer le port 80 pour accéder à l'application via HTTP
EXPOSE 80

# Lancer NGINX
CMD ["nginx", "-g", "daemon off;"]