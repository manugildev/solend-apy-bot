<template>
    <b-row class="market-row" >
        <b-col class="market-token" cols="4">
            <img class="market-icon" :src="require(`@/assets/logo_${apy.asset.toLowerCase()}.png`)"/>
            <div class="market-token-col">
              <div class="market-token-name">{{ apy.name }}</div>
              <div class="market-token-price">{{ format_currency_value(apy.price) }}</div>
            </div>
        </b-col>

        <b-col v-if="apy.weight_supply == 0" class="market-apy-container market-apy" cols="4">{{ format_percent_value(apy.supply) }}</b-col>
        <b-col v-else class="market-apy-container" cols="4">
          <div>
            <div class="market-boost" v-if="apy.weight_supply != 0 && apy.asset != 'mSOL'">{{ apy.weight_supply }}</div>
            <div class="market-boost market-boost-mnde" v-if="apy.weight_supply != 0 && apy.asset == 'mSOL'">{{ apy.weight_borrow + apy.weight_supply }} + 
              <img alt="mnde token" class="market-slnd-token" src="@/assets/logo_mnde.png">
            </div>

            <span class="market-apy"> {{ format_percent_value(apy.supply) }} </span> <br/>
            <span class="market-reward">
              ( {{ format_reward_value(apy.supply_rewards) }} <img alt="solend token" class="market-slnd-token" src="@/assets/logo_slnd.png"> 
                {{ apy.asset == 'mSOL' ? '+' : ''}} <img v-if="apy.asset=='mSOL'" alt="solend token" class="market-slnd-token" src="@/assets/logo_mnde.png"> 
              )
            </span>
          </div>
        </b-col>

        <b-col v-if="apy.weight_borrow != 0" class="market-apy-container" cols="4" v-bind:class="{'market-borrow-left' : left}">
          <div>
            <div class="market-boost" v-if="apy.weight_borrow != 0 && apy.asset != 'mSOL'">{{ apy.weight_borrow }}</div>
            <span class="market-apy"> {{ format_percent_value(apy.borrow) }} </span> <br/>
            <span class="market-reward">
              ( {{ format_reward_value(apy.borrow_rewards) }} <img alt="solend token" class="market-slnd-token" src="@/assets/logo_slnd.png"> )
            </span>
          </div>
        </b-col>
        <b-col v-else class="market-apy-container market-apy" v-bind:class="{'market-borrow-left' : left}" cols="4">
          {{ format_percent_value(apy.borrow) }}
        </b-col>
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
        },
        format_reward_value: function(value){
            if(!value) { return 0}
            return (parseFloat(value * 100).toFixed(1)) + "%"
        },
    }
};
</script>

<style>

.market-token {
  align-items: center;
  display: flex;
  justify-content: left;
}

.market-borrow-left {
  border-right: 1px solid #23242E;
}

.market-icon {
  border-radius: 100%;
  height: 45px;
  width: 45px;
  margin: 0px 10px;
  min-height: 45px;
  min-width: 45px;
  overflow: hidden;
}

.market-token-col {
  padding-left: 5px;
  text-align: left!important;
  width: 100%;
}

.market-token-name {
  color: #FEFEFE;
  display: inline-block;
  font-size: 20px;
}

.market-token-price {
  color: #64676D;
  font-size: 16px;
}

.market-apy-container {
  align-items: center;
  color: #FEFEFE;
  display: grid;
  justify-content: right;
  text-align: right!important;
}

.market-apy {
  display: inline-block;
  font-size: 17px;
  min-width: 70px;
}

.market-reward {
  color: #64676D;
  font-size: 14px;
}

.market-slnd-token {
  border-radius: 100%;
  height: 15px;
  width: 15px;
  margin-bottom: 1px;
  overflow: hidden;
}

.market-boost {
  background: linear-gradient(275.27deg,#FF5C28 1.51%,#FFE600 195.89%);
  border-radius: 4px;
  color: #FFFFFF;
  display: inline-block;
  font-size: 16px;
  font-weight: bold;
  text-shadow: 0px 0px 6px #4D4D4D, 1px 1px 0px #3B3B3B;
  display: inline-block;
  margin-right: 6px;
  padding: 0 8px;
}

.market-boost-mnde {
  background: linear-gradient(275.27deg, rgb(202, 41, 240) 1.51%, rgb(255, 230, 0) 195.89%);
  min-width: 60px;
  white-space: nowrap;
  margin-right: -12px;
  padding: 0 8px;
}
</style>