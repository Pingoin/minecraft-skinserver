<template>
  <v-container class="pa-4">
    <v-card class="pa-4" elevation="2">
      <v-card-title>Hallo Vuetify 🎨</v-card-title>
      <v-card-text>
        <p>Dies ist eine Testkarte mit SCSS-Style.</p>
      </v-card-text>
      <v-btn color="primary" @click="clicked">Klick mich</v-btn>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
function clicked() {
  alert('Vuetify funktioniert!')
}
</script>

<style lang="scss" scoped>
@import '../styles/main.scss';
.v-card {
  background-color: #f5f5f5;
  border-radius: 12px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}
p{
  color: $primary-color;
}
</style>