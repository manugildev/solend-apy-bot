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
  background-color: #0F1018;
  background-image: url(../../assets/market_hero.dark.svg);
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 100vh;
  align-items: center;
  justify-items: center;
}

#app {
  font-family: "IBM Plex Sans",-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans",sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji";
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  min-width: 1000px;
}

</style>
