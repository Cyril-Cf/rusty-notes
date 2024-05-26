<template>
  <v-parallax src="/images/background.jpg" height="auto">
    <v-container>
      <v-card class="mx-auto my-12" max-width="800">
        <v-card-title class="text-center">
          <h1>Bienvenue sur ton site de gestion de notes !</h1>
        </v-card-title>
        <v-card-text>
          <p>
            Ce site te permet de créer, organiser et gérer tes notes de manière simple et efficace.
          </p>
          <p>
            Voici quelques fonctionnalités :
          </p>
          <ul>
            <li>Crée des notes et organise-les par catégories</li>
            <li>Marque tes notes comme complétées</li>
            <li>Ajoute des tâches et des rappels à tes notes</li>
            <li>Synchronise tes notes sur tous tes appareils</li>
          </ul>
          <p>
            Utilise le menu de navigation pour accéder aux différentes sections de l'application.
          </p>

          <h2>Sous-section 1</h2>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse varius enim in eros elementum
            tristique. Duis cursus, mi quis viverra ornare, eros dolor interdum nulla, ut commodo diam libero vitae
            erat.
          </p>
          <h2>Sous-section 2</h2>
          <p>
            Aenean faucibus nibh et justo cursus id rutrum lorem imperdiet. Nunc ut sem vitae risus tristique posuere.
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse varius enim in eros elementum
            tristique.
          </p>
          <h2>Sous-section 3</h2>
          <p>
            Duis cursus, mi quis viverra ornare, eros dolor interdum nulla, ut commodo diam libero vitae erat. Aenean
            faucibus nibh et justo cursus id rutrum lorem imperdiet. Nunc ut sem vitae risus tristique posuere.
          </p>
          <h2>Sous-section 4</h2>
          <p>
            Suspendisse varius enim in eros elementum tristique. Duis cursus, mi quis viverra ornare, eros dolor
            interdum nulla, ut commodo diam libero vitae erat. Aenean faucibus nibh et justo cursus id rutrum lorem
            imperdiet.
          </p>
          <h2>Sous-section 5</h2>
          <p>
            Nunc ut sem vitae risus tristique posuere. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            Suspendisse varius enim in eros elementum tristique. Duis cursus, mi quis viverra ornare, eros dolor
            interdum nulla, ut commodo diam libero vitae erat.
          </p>
          <h2>Sous-section 6</h2>
          <p>
            Aenean faucibus nibh et justo cursus id rutrum lorem imperdiet. Nunc ut sem vitae risus tristique posuere.
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse varius enim in eros elementum
            tristique.
          </p>
          <h2>Sous-section 7</h2>
          <p>
            Duis cursus, mi quis viverra ornare, eros dolor interdum nulla, ut commodo diam libero vitae erat. Aenean
            faucibus nibh et justo cursus id rutrum lorem imperdiet. Nunc ut sem vitae risus tristique posuere.
          </p>
          <h2>Sous-section 8</h2>
          <p>
            Suspendisse varius enim in eros elementum tristique. Duis cursus, mi quis viverra ornare, eros dolor
            interdum nulla, ut commodo diam libero vitae erat. Aenean faucibus nibh et justo cursus id rutrum lorem
            imperdiet.
          </p>
        </v-card-text>
        <v-card-actions class="text-center">
          <v-btn color="primary" @click="getStarted">Commencer</v-btn>
        </v-card-actions>
      </v-card>
    </v-container>
  </v-parallax>
</template>

<script lang="ts" setup>
import router from "@/router";
const getStarted = () => {
  router.push({ path: "/my_lists" });
};
</script>

<style scoped>
p {
  margin: 3rem 0;
}

.my-12 {
  margin-top: 3rem;
  margin-bottom: 3rem;
}
</style>
