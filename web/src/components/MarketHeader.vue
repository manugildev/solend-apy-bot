<template>
  <b-row class="header">
    <b-col class="header-col" v-for="index in 2" :key="index">
      <b-row class="header-container">
        <b-col class="header-token" cols="6">
          <span class="header-token-text">Market name</span>
        </b-col>
        <b-col class="header-apy" cols="3">Supply APY</b-col>
        <b-col class="header-apy" :class="{'header-borrow-left' : index == 1}" cols="3">Borrow APY</b-col>
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
  display: flex;
  align-items: flex-end;
}

.header {
  height: 14%;
  color: #64676d;
  font-size: 16px;
}

.header-col {
  border-bottom: 1px solid #23242e;
  height: 100%;
  display: grid;
  align-items: center;
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
  border-right: 1px solid #23242e;

}
</style>