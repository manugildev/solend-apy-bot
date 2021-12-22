<template>
  <b-row class="header">
    <b-col class="header-col" v-for="index in 2" :key="index">
      <b-row class="header-container">
        <b-col class="header-token" cols="4">
          <span class="header-token-text">Market name</span>
        </b-col>
        <b-col class="header-apy" cols="4">Supply APY</b-col>
        <b-col class="header-apy" :class="{'header-borrow-left' : index == 1}" cols="4">Borrow APY</b-col>
      </b-row>
    </b-col>
  </b-row>
</template>

<script>
export default {
  name: "MarketHeader",
  props: {
  }
};
</script>

<style>
.header-container {
  align-items: flex-end;
  display: flex;
}

.header {
  color: #64676D;
  font-size: 16px;
  height: 8%;
}

.header-col {
  align-items: center;
  border-bottom: 1px solid #23242E;
  display: grid;
  height: 100%;
}

.header-token {
  text-align: left !important;
}

.header-token-text {
  padding-left: 5px;
}

.header-apy {
  text-align: right !important;
}

.header-borrow-left {
  border-right: 1px solid #23242E;
}
</style>