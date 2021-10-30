<template>
    <b-row class="market-row" >
        <b-col class="market-token" cols="6">
            <img class="market-icon" :src="require(`@/assets/logo_${apy.asset.toLowerCase()}.png`)"/>
            <div class="market-token-col">
              <div class="market-token-name">{{ apy.name }}</div>
              <div class="market-token-price">{{ format_currency_value(apy.price) }}</div>
            </div>
            <div class="market-boost" v-if="format_reward_value(apy.borrow_rewards) != 0">{{apy.weight}}x</div>
        </b-col>

        <b-col v-if="format_reward_value(apy.supply_rewards) != 0" class="market-apy-container" cols="3">
          <div>
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
  align-items: center;
  display: flex;
  justify-content: left;
}

.market-borrow-left {
  border-right: 1px solid #23242E;
}

.market-icon {
  border-radius: 100%;
  height: 50px;
  margin: 0px 10px;
  min-height: 50px;
  min-width: 50px;
  overflow: hidden;
  width: 50px;
}

.market-token-col {
  padding-left: 10px;
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
  font-size: 19px;
  min-width: 65px;
}

.market-reward {
  color: #64676D;
  font-size: 14px;
}

.market-slnd-token {
  border-radius: 100%;
  height: 12px;
  margin-bottom: 1px;
  outline: 1px solid rgba(255, 255, 255, 0.2);
  overflow: hidden;
  width: 12px;
}

.market-boost {
  background: linear-gradient(275.27deg,#FF5C28 1.51%,#FFE600 195.89%);
  border-radius: 4px;
  color: #FFFFFF;
  display: inline-block;
  font-size: 19px;
  font-weight: bold;
  margin-right: -15px;
  padding: 0px 6px;
  text-shadow: 0px 0px 6px #4D4D4D, 1px 1px 0px #3B3B3B;
}
</style>