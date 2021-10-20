<template>
    <b-row class="market-row" >
        <b-col class="market-token" cols="6">
            <img class="market-icon" :src="require(`@/assets/logo_${apy.asset.toLowerCase()}.png`)"/>
            <div class="market-token-col">
              <div class="market-token-name">{{ apy.asset }}</div>
              <div class="market-token-price">{{ format_currency_value(apy.price) }}</div>
            </div>
        </b-col>

        <b-col v-if="format_reward_value(apy.supply_rewards) != 0" class="market-apy-container" cols="3">
          <div>
            <div class="market-boost">{{apy.weight}}x</div>
            <span class="market-apy"> {{ format_percent_value(apy.supply) }} </span> <br/>
            <span class="market-reward">
              {{ format_reward_value(apy.supply_rewards) }}
              <img alt="solend token" class="market-slnd-token" src="@/assets/logo_slnd.png">
              / $1k
            </span>
          </div>
        </b-col>

        <b-col v-if="format_reward_value(apy.borrow_rewards) != 0" class="market-apy-container" cols="3" v-bind:class="{'market-borrow-left' : left}">
          <div>
            <div class="market-boost">{{apy.weight}}x</div>
            <span class="market-apy"> {{ format_percent_value(apy.borrow) }} </span> <br/>
            <span class="market-reward">
              {{ format_reward_value(apy.borrow_rewards) }}
              <img alt="solend token" class="market-slnd-token" src="@/assets/logo_slnd.png">
              / $1k
            </span>
          </div>
        </b-col>

        <!-- IF REWARDS 0.00 -->
        <b-col v-if="format_reward_value(apy.supply_rewards) == 0" class="market-apy-container market-apy" cols="3">{{ format_percent_value(apy.supply) }}</b-col>
        <b-col v-if="format_reward_value(apy.supply_rewards) == 0" class="market-apy-container market-apy" v-bind:class="{'market-borrow-left' : left}" cols="3">
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
            return "+ " + (parseFloat(value).toFixed(1))
        },
    }
};
</script>

<style>

.market-token {
  display: flex;
  align-items: center;
  justify-content: left;
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

.market-apy-container {
  display: grid;
  align-items: center;
  justify-content: right;
  text-align: right!important;
  color: #FEFEFE;
}

.market-apy {
 font-size: 17px;
 min-width: 65px;
 display: inline-block;
}

.market-reward {
  color: #FEFEFE;
  font-size: 14px;
  color: #64676d
}

.market-slnd-token {
  height: 12px;
  width: 12px;
  border-radius: 100%;
  overflow: hidden;
  margin-bottom: 1px;
}

.market-boost {
    background: linear-gradient(275.27deg,#E05E34 1.51%,#C45D3C 195.89%);
    color: #fff;
    border-radius: 4px;
    display: inline-block;
    padding: 0 4px;
    margin-right: 5px;
    font-size: 16px;
    font-weight: bold;
}
</style>