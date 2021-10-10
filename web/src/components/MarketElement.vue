<template>
    <b-row class="market-row" >
        <b-col class="market-token" cols="6">
            <img class="market-icon" :src="require(`@/assets/logo_${apy.asset.toLowerCase()}.png`)"/>
            <div class="market-token-col">
            <div class="market-token-name">{{ apy.asset }}</div>
            <div class="market-token-price">{{ format_currency_value(apy.price) }}</div>
            </div>
        </b-col>
        <b-col class="market-apy" cols="3">{{ format_percent_value(apy.supply) }}</b-col>
        <b-col class="market-apy" v-bind:class="{'market-borrow-left' : left}" cols="3">{{ format_percent_value(apy.borrow) }}</b-col>
    </b-row>
</template>

<script>
export default {
  name: "MarketElement",
  props: {
      apy: Object,
      left: Boolean,
  },
  methods: {
        format_percent_value: function(value){
            if(!value) { return "0.00%"}
            return (parseFloat(value * 100).toFixed(2)) + "%"
        },
        format_currency_value: function(value){
            if(!value) { return "$0.00"}
            return "$" + (parseFloat(value).toFixed(2))
        }
    }
};
</script>

<style>

.main-row {
  height: 100%;
}

.market-token {
  display: flex;
  align-items: center;
  justify-content: left;
  margin: 6px 0px 6px 0px;
}

.market-borrow-left {
  border-right: 1px solid #23242E;
}

.market-icon {
  height: 50px;
  width: 50px;
  border-radius: 100%;
  overflow: hidden;
  margin: 0px 10px;
}

.market-token-col {
  padding-left: 10px;
  text-align: left!important;
}

.market-token-name {
  display: inline-block;
  color: #FEFEFE;
}

.market-token-price {
  color: #FEFEFE;
  font-size: 16px;
  color: #64676d;
}

.market-apy {
  height: 70%;
  display: flex;
  align-items: center;
  justify-content: right;
  text-align: right!important;
  color: #FEFEFE;
}
</style>