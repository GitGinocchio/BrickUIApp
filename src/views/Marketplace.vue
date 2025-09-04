<template>
  <div class="container">
    <n-tabs type="line">
      <n-tab-pane name="bricks" tab="Bricks">
        <div class="brick-grid">
          <BrickCard
            v-for="brick in bricks"
            :key="brick.name"
            :brick="brick"
          />
        </div>
      </n-tab-pane>
      <n-tab-pane name="walls" tab="Walls">
        <p>Qui si possono scaricare temi.</p>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import BrickCard from '../components/marketplace/BrickCard.vue';
import { NTabs, NTabPane } from "naive-ui"
const bricks = [
  {
    name: "Brick 1",
    description: "Questo brick è uno strumento completo per la gestione dei progetti, progettato per aiutare team di qualsiasi dimensione a coordinare attività, scadenze e risorse in maniera semplice ed efficiente. Include dashboard personalizzabili, notifiche in tempo reale, report dettagliati e integrazioni con le principali piattaforme di lavoro collaborativo. Ideale per team agili che vogliono tenere traccia di ogni task e ottenere una panoramica chiara dello stato di avanzamento dei progetti.",
    tags: ["Utility", "Stable", "Popular"],
    version: "1.0.0",
    link: "https://example.com",
    icon: "https://via.placeholder.com/32",
  },
  {
    name: "Brick 2",
    description: "Un brick potente per l’analisi dei dati, che permette di trasformare grandi quantità di informazioni in insight utili. Offre visualizzazioni interattive, grafici dinamici, filtri avanzati e la possibilità di esportare report in diversi formati. Grazie agli algoritmi integrati di machine learning, è possibile rilevare trend nascosti e supportare decisioni strategiche basate sui dati.",
    tags: ["AI", "Beta"],
    version: "2.3.1",
    price: 25,
    active: false,
    link: "https://example.com",
    icon: "https://via.placeholder.com/32",
  }
]
</script>

<style scoped>
.brick-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
  gap: 1rem
}

.container { 
  padding: 16px;
}
</style>