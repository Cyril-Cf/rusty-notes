# Utilisation d'une image Node.js
FROM node:21

# Définir le répertoire de travail dans le conteneur
WORKDIR /app

# Copier les fichiers de l'application
COPY . .

# Installer les dépendances
RUN npm install

# Exposer le port 3000
EXPOSE 3000

# Commande par défaut pour démarrer l'application
CMD ["npm", "run", "dev"]
