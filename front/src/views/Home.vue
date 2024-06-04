<template>
  <section id="title" class="my-15">
    <div class="d-flex justify-center align-center text-center px-5 px-lg-0 mb-4 mb-lg-0">
      <div class="text-h4">Mes notes partagées</div>
    </div>
    <div class="d-flex justify-center text-justify font-weight-light text-body-1">
      <div class="my-10" style="width: 70vw">
        <p>
          <span class="app_name">Rusty Notes</span> est votre plateforme ultime pour collaborer, organiser et partager
          vos
          notes en temps réel avec vos
          amis et collègues.
        </p>
        <br>
        <p>
          Que vous soyez en train de planifier un projet, d'échanger des idées créatives ou de
          simplement prendre des notes de cours, <span class="app_name">Rusty Notes</span> vous permet de rester
          connecté
          et synchronisé. Profitez
          d'une
          interface intuitive, de mises à jour instantanées et de fonctionnalités puissantes pour transformer votre
          façon
          de travailler ensemble.
        </p>
        <br>
        <p>
          Rejoignez la communauté <span class="app_name">Rusty Notes</span> et révolutionnez votre
          collaboration dès aujourd'hui
          !
        </p>
      </div>
    </div>
    <div class="d-flex justify-center">
      <v-btn color="primary" @click="getStarted">Commencer</v-btn>
    </div>
  </section>
  <section id="illustration">
    <v-parallax src="/images/smile.avif" :height="mdAndUp ? '600' : '400'"
      class="text-white d-flex justify-center align-center">
      <div class="gradient-overlay"></div>
      <div class="d-flex justify-center text-container">
        <div :class="mdAndUp ? 'text-h3 text-center' : 'text-h5 text-center'">Rusty Notes : L'art de la Collaboration
          Instantanée</div>
      </div>
      <div class="d-flex justify-center text-justify font-weight-light text-body-1 text-container">
        <div class="my-4 text-body-1 text-center" :style="mdAndUp ? 'width: 50vw;' : 'width: 80vw;'">
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
    <div class="d-flex justify-center align-center flex-wrap mx-4 mx-lg-16 mt-4">
      <v-card v-for="card in cards" :key="card.title" class="my-4 my-lg-0 mx-md-4" :prepend-icon="card.icon"
        width="400px">
        <template v-slot:title>
          <span class="font-weight-black text-wrap" style="word-break: break-word;">{{ card.title }}</span>
        </template>
        <v-card-text class="bg-surface-light pt-4">
          <div class="d-flex justify-center align-center flex-wrap">
            <v-chip v-for="tag in card.tags" :key="tag.name" class="mb-4 mb-md-0 mr-0 mr-sm-1" :prepend-icon="tag.icon"
              :color="tag.color">
              {{ tag.name }}
            </v-chip>
          </div>
          <v-list>
            <v-list-item v-for="item in card.items" :key="item.title">
              <v-list-item-title class="text-wrap mt-4 mt-md-0 mb-2 mb-md-0">{{ item.title }}</v-list-item-title>
              <v-list-item-subtitle class="text-wrap" style="display: block;">
                {{ item.subtitle }}
              </v-list-item-subtitle>
            </v-list-item>
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
        <v-col sm="10" md="8" class="text-h4 text-center mt-6">Foire Aux Questions (FAQ)</v-col>
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
                serveur privé dont la continuité de service n'est pas garanti (à ce jour en tout cas, cela pourrait
                changer si cette
                plateforme
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
import { useDisplay } from 'vuetify'
const { mdAndUp } = useDisplay()
const getStarted = () => {
  router.push({ path: "/my_notes" });
};
const cards = [
  {
    title: "Créez des notes à l'infini",
    icon: "mdi-note-multiple",
    tags: [
      {
        name: "Notes Illimitées",
        icon: "mdi-playlist-plus",
        color: "primary"
      },
      {
        name: "Organisation Simplifiée",
        icon: "mdi-format-list-bulleted",
        color: "primary"
      },
    ],
    items: [
      {
        title: "Gestion sans effort",
        subtitle: "Créez et gérez vos notes en quelques clics."
      },
      {
        title: "Personnalisation avancée",
        subtitle: "Ajoutez des tags pour différencier vos notes."
      },
      {
        title: "Partage facile",
        subtitle: "Partagez vos notes et gérez les permissions."
      },
    ]
  },
  {
    title: "Collaborez entre amis",
    icon: "mdi-account-group",
    tags: [
      {
        name: "Travail d'Équipe",
        icon: "mdi-account-multiple",
        color: "primary"
      },
      {
        name: "Suivi en Temps Réel",
        icon: "mdi-progress-check",
        color: "secondary"
      },
    ],
    items: [
      {
        title: "Communication instantanée",
        subtitle: "Échangez des idées en temps réel."
      },
      {
        title: "Suivi collaboratif",
        subtitle: "Suivez les contributions de chacun."
      },
      {
        title: "Travail d'équipe facilité",
        subtitle: "Soyez alertés par les notifications intégrées."
      },
    ]
  },
  {
    title: "Profitez du travail synchronisé",
    icon: "mdi-sync",
    tags: [
      {
        name: "Synchronisation",
        icon: "mdi-cloud-sync",
        color: "secondary   "
      },
      {
        name: "Accès Global",
        icon: "mdi-access-point-network",
        color: "primary"
      },
    ],
    items: [
      {
        title: "Synchronisation instantanée",
        subtitle: "Vos notes sont mises à jour en temps réel."
      },
      {
        title: "Accès partout",
        subtitle: "Retrouvez vos notes où et quand vous le souhaitez."
      },
      {
        title: "Sécurité optimale",
        subtitle: "Vos données sont protégées et sécurisées."
      },
    ]
  },
]
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
