<template>
  <section id="title" class="my-15">
    <div class="d-flex justify-center">
      <div class="text-h4">Mes notes partagées</div>
    </div>
    <div class="d-flex justify-center text-justify font-weight-light text-body-1">
      <div class="my-4" style="width: 50vw">
        <span class="app_name">Rusty Notes</span> est votre plateforme ultime pour collaborer, organiser et partager vos
        notes en temps réel avec vos
        amis et collègues. Que vous soyez en train de planifier un projet, d'échanger des idées créatives ou de
        simplement prendre des notes de cours, <span class="app_name">Rusty Notes</span> vous permet de rester connecté
        et synchronisé. Profitez
        d'une
        interface intuitive, de mises à jour instantanées et de fonctionnalités puissantes pour transformer votre
        façon
        de travailler ensemble. Rejoignez la communauté <span class="app_name">Rusty Notes</span> et révolutionnez votre
        collaboration dès aujourd'hui
        !
      </div>
    </div>
    <div class="d-flex justify-center">
      <v-btn color="primary" @click="getStarted">Commencer</v-btn>
    </div>
  </section>
  <section id="illustration">
    <v-parallax src="/images/smile.avif" height="600px" class="text-white d-flex justify-center align-center">
      <div class="gradient-overlay"></div>
      <div class="d-flex justify-center text-container">
        <div class="text-h3 text-center">Rusty Notes : L'art de la Collaboration Instantanée</div>
      </div>
      <div class="d-flex justify-center text-justify font-weight-light text-body-1 text-container">
        <div class="my-4 text-body-1 text-center" style="width: 50vw">
          Écrite en Rust <v-icon>mdi-language-rust</v-icon>, notre application révolutionne la gestion de notes
          partagées en temps réel. Connectez-vous, collaborez et créez ensemble, sans compromis et gratuitement !
        </div>
      </div>
    </v-parallax>
  </section>
  <section id="use_case" class="my-10">
    <div class="d-flex justify-center">
      <div class="text-h4">Nos services rendus</div>
    </div>
    <v-container>
      <v-row>
        <v-col>
          <v-divider color="primary" thickness="1"></v-divider>
        </v-col>
      </v-row>
    </v-container>
    <div class="d-flex justify-center align-center mx-16 mt-4">
      <v-card class="mx-4" prepend-icon="mdi-playlist-plus" subtitle="The #1 Vue UI Library" width="400">
        <template v-slot:title>
          <span class="font-weight-black">Créez des listes à l'infini</span>
        </template>
        <v-card-text class="bg-surface-light pt-4">
          <div class="d-flex justify-center align-center">
            <v-chip prepend-icon="mdi-brightness-5" color="primary">
              Listes cools
            </v-chip>
            <v-chip prepend-icon="mdi-alarm-check">
              Items cools
            </v-chip>
            <v-chip prepend-icon="mdi-alarm-check">
              Suivi cools
            </v-chip>
          </div>
          <v-list>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
      <v-card class="mx-4" prepend-icon="mdi-account-group" subtitle="The #1 Vue UI Library" width="400">
        <template v-slot:title>
          <span class="font-weight-black">Collaborez entre amis</span>
        </template>
        <v-card-text class="bg-surface-light pt-4">
          <v-list>
            <div class="d-flex justify-center align-center">
              <v-chip prepend-icon="mdi-brightness-5">
                Listes cools
              </v-chip>
              <v-chip prepend-icon="mdi-alarm-check">
                Items cools
              </v-chip>
              <v-chip prepend-icon="mdi-alarm-check">
                Suivi cools
              </v-chip>
            </div>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
      <v-card class="mx-4" prepend-icon="mdi-sync" subtitle="The #1 Vue UI Library" width="400">
        <template v-slot:title>
          <span class="font-weight-black">Profitez du travail synchronisé</span>
        </template>
        <v-card-text class="bg-surface-light pt-4">
          <v-list>
            <div class="d-flex justify-center align-center">
              <v-chip prepend-icon="mdi-brightness-5">
                Listes cools
              </v-chip>
              <v-chip prepend-icon="mdi-alarm-check">
                Items cools
              </v-chip>
              <v-chip prepend-icon="mdi-alarm-check">
                Suivi cools
              </v-chip>
            </div>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
            <v-list-item title="Item" subtitle="Lorem ipsum dolor sit amet consectetur adipisicing elit"></v-list-item>
          </v-list>
        </v-card-text>
      </v-card>
    </div>
  </section>
  <section id="ready">
    <v-parallax src="/images/fluids.avif" height="300px" class="text-white d-flex justify-center align-center">
      <div class="gradient-overlay"></div>
      <div class="d-flex justify-center text-container">
        <div class="text-h4 text-center">Prêt(e) à tester ?</div>
      </div>
      <div class="d-flex justify-center text-container my-5">
        <div class="d-flex justify-center">
          <v-btn color="primary" @click="getStarted">Je me lance !</v-btn>
        </div>
      </div>
    </v-parallax>
  </section>
  <section id="faq" class="mb-10">
    <v-container>
      <v-row justify="center">
        <v-col cols="6 text-h4 text-center mt-6">Foire Aux Questions (FAQ)</v-col>
      </v-row>
      <v-row justify="center">
        <v-col cols="10">
          <v-expansion-panels variant="accordion">
            <v-expansion-panel title="Qui est derrière Rusty Notes ?">
              <template v-slot:text>
                Cette application a été créée et maintenue par <a href="https://formation-rust-web.fr"
                  target="_blank">Cyril Couffe</a>, développeur fullstack spécialisé en Rust, dans le cadre initial d'un
                travail personnel d'exploration et d'amusement, qui s'est poursuivi jusqu'à une application totalement
                opérationnelle.
              </template>
            </v-expansion-panel>
          </v-expansion-panels>
          <v-expansion-panels variant="accordion">
            <v-expansion-panel title="Combien coûte l'utilisation ?"
              text="Absolument rien. Il n'est pas prévu de faire de cette application une source de revenus.">
            </v-expansion-panel>
          </v-expansion-panels>
          <v-expansion-panels variant="accordion">
            <v-expansion-panel title="Dois-je être prudent(e) avec les informations que j'enregistre ici ?">
              <template v-slot:text>
                Comme il s'agit d'un travail personnel à des fins de démonstration, les données sont stockées sur un
                serveur privé dont la continuité de service n'est pas garanti (à ce jour, sauf si cette plateforme
                devient très utilisée). Soyez donc prudent avec l'usage et la dépendance à ce service !
              </template>
            </v-expansion-panel>
          </v-expansion-panels>
          <v-expansion-panels variant="accordion">
            <v-expansion-panel title="Comment nous remercier pour les services rendus ?">
              <template v-slot:text>
                N'hésitez pas à visiter <a href="https://formation-rust-web.fr/contact.html" target="_blank">mon site
                  web</a>, un formulaire de contact vous
                attends
                pour échanger !
              </template>
            </v-expansion-panel>
          </v-expansion-panels>
        </v-col>
      </v-row>
    </v-container>
  </section>
</template>

<script lang="ts" setup>
import router from "@/router";
const getStarted = () => {
  router.push({ path: "/my_notes" });
};
</script>

<style scoped>
.gradient-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(to bottom, rgba(0, 0, 0, 0.8), rgba(0, 0, 0, 0.8));
  z-index: 1;
}

.text-container {
  position: relative;
  z-index: 2;
}

.app_name {
  color: rgb(var(--v-theme-primary));
  font-weight: bold;
}

a,
a:visited,
a:hover {
  color: rgb(var(--v-theme-primary));
}
</style>
