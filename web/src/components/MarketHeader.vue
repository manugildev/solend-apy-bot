<template>
  <b-row class="market-header">
    <b-col class="market-header-col" v-for="index in 2" :key="index">
      <b-row class="market-header-container">
        <b-col class="market-header-token" cols="6"> Market name </b-col>
        <b-col class="market-header-apy" cols="3">Supply APY</b-col>
        <b-col class="market-header-apy" cols="3">Borrow APY</b-col>
      </b-row>
    </b-col>
  </b-row>
</template>

<script>
export default {
  name: "MarketHeader",
};
</script>

<style>
.market-header-container {
  display: flex;
  align-items: flex-end;
}

.market-header {
  height: 14%;
  color: #64676d;
  font-size: 16px;
  display: flex;
  align-items: flex-end;
  margin-bottom: 8px;
}

.market-header-col {
  border-bottom: 1px solid #23242e;
}

.market-header-token {
  text-align: left !important;
  padding-bottom: 3px;
}

.market-header-apy {
  text-align: right !important;
  padding-bottom: 3px;
}
</style>