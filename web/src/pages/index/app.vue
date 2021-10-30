<template>
  <div id="app">
    <!-- <img alt="Vue logo" src="./assets/logo.png"> -->
    <MarketData />
  </div>
</template>

<script>
import MarketData from '@/components/MarketData.vue'

export default {
  name: 'App',
  components: { MarketData }
}
</script>

<style>
body {
  align-items: center;
  background-color: #0F1018;
  background-image: url(../../assets/market_hero.dark.svg);
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 100vh;
  justify-items: center;
}

#app {
  color: #2C3E50;
  font-family: "IBM Plex Sans",-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans",sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji";
  -moz-osx-font-smoothing: grayscale;
  -webkit-font-smoothing: antialiased;
  min-width: 1000px;
}

</style>
