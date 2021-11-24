<template>
    <div class="main-container">
      <b-aspect class="vld-parent" :aspect="'16:9'">
        <loading :active.sync="is_loading"
                 :is-full-page="full_page"
                 :loader="loader"
                 :opacity="opacity"
                 :background-color="backgroundColor"
                 :color="color"
                 :class="[is_loading ? 'not-loaded' : 'loaded']"></loading>
        <b-container id="markets-container">
          <MarketHeader/>
          <b-row class="main-row">
            <!-- Left Column -->
            <b-col cols="6">
              <MarketElement v-for="apy in apys.slice(0, Math.ceil(apys.length / 2))" :key="apy" v-bind:apy="apy" left />
            </b-col>
            <!-- Right Column -->
            <b-col cols="6">
              <MarketElement v-for="apy in apys.slice( Math.ceil(apys.length / 2), apys.length)" :key="apy" v-bind:apy="apy"/>
            </b-col>
          </b-row>
        </b-container>
        <b-container id="stats-container">
          <b-row class="stats-row">

            <b-col cols="3" class="separator-left">
              <div class="stats-col">
                <div class="stats-name">Total supply</div>
                <div class="stats-value">{{ format_millions_value(info.total_supplied) }}</div>
              </div>
            </b-col>

            <b-col cols="3" class="separator-left">
              <div class="stats-col">
                <div class="stats-name">Total borrow</div>
                <div class="stats-value">{{ format_millions_value(info.total_borrowed) }}</div>
              </div>
            </b-col>

            <b-col cols="3" class="separator-left">
              <div class="stats-col">
                <div class="stats-name">TVL</div>
                <div class="stats-value">{{ format_millions_value(info.total_supplied-info.total_borrowed) }}</div>
              </div>
            </b-col>

            <b-col cols="3" class="stats-token-container">
              <img class="stats-logo" :src="require(`@/assets/logo_slnd.png`)"/>
              <div class="stats-col" style="padding-left: 10px;">
                <div class="stats-name">SLND price<!--(<span class="stats-ido">IDO</span>)--></div>
                <div class="stats-value">{{ format_currency_value(info.slnd_price) }}</div>
              </div>
            </b-col>

          </b-row>
        </b-container>
      </b-aspect>
    <!--
    <b-container class="button-container">
      <b-row align-h="end">
        <b-button class="image-button" squared v-on:click="take_screenshot"
                  :style="{visibility: is_loading ? 'hidden' : 'visible'}">
          <b-icon icon="image-fill" font-scale="1"></b-icon>
          <span class="button-text"> Save Image</span>
        </b-button>
      </b-row>
    </b-container>
    -->

  </div>
</template>

<script>
import Vue from "vue";
import MarketHeader from "./MarketHeader";
import MarketElement from "./MarketElement";
import abbreviate from "number-abbreviate";
import Loading from "vue-loading-overlay";
import "vue-loading-overlay/dist/vue-loading.css";


export default {
  name: "MarketData",
  components: { MarketHeader, MarketElement, Loading, },
  data() {
    return {
      info: {},
      info_data_cache: "{ 'total_supplied' : 0.0, 'total_borrowed': '0.0', 'slnd_price': '0.0'}",
      apys: [],
      table_data_cache: "",
      backgroundColor: "#0E1118",
      color: "#FF5C28",
      full_page: false,
      is_loading: true,
      loader: "dots",
      opacity: 0.9,
    };
  },
  mounted() {
    if (localStorage.table_data_cache) { this.apys = JSON.parse(localStorage.table_data_cache); }
    if (localStorage.info_data_cache) { this.info = JSON.parse(localStorage.info_data_cache); }
  },
  watch: {
    table_data_cache(new_data) { localStorage.table_data_cache = new_data; },
    info_data_cache(new_data) { localStorage.info_data_cache = new_data; },
  },
  async created() {
    this.is_loading = !Vue.config.devtools;

    if (Vue.config.devtools) return;

    // GET /apy request using fetch with async/await
    let response = await fetch("/apy");
    let table_data = await response.json();
    this.table_data_cache = JSON.stringify(table_data);
    this.apys = table_data;

    // GET /info request using fetch with async/await
    response = await fetch("/info");
    let info_data = await response.json();
    this.info_data_cache = JSON.stringify(info_data);
    this.info = info_data;

    this.is_loading = false;
  },

  methods: {
      format_currency_value: function(value){
          if(!value) { return "$0.00"}
          return "$" + (parseFloat(value).toFixed(2))
      },
      format_millions_value: function(value){
          if(!value) { return "$0M"}

          var decimals = value > 999999999 ? 2: 0;
          return ("$" + abbreviate(value, decimals)).toUpperCase();
      },

    /*  take_screenshot: function () {
          let markets_container = document.getElementById('markets-container');
          html2canvas(markets_container).then(function(canvas) {
            let a = document.createElement('a');
            a.href = canvas.toDataURL("image/jpeg").replace("image/jpeg", "image/octet-stream");
            let date = moment(new Date()).format("YYYYMMDD-hhmmss");
            a.download = "solend_apy_screenshot_" + date + ".jpeg";
            a.click();
          });
        },
*/
  },
};
</script>

<style scoped>
#stats-container {
  align-items: center;
  background-color: #0F1018;
  border: 1px solid #23242E;
  border-top: 0px;
  display: grid;
  font-size: 18px;
  height: 13%;
  padding: 0px 20px 0px 20px;
}

.stats-col {
  text-align: center!important;
  justify-content: center;

}

.stats-name {
  color: #64676D;
  display: inline-block;
  font-size: 16px;
}

.stats-token-container {
  align-items: center;
  display: flex;
  justify-content: center;
  text-align: center!important;
}

.stats-ido {
  background: -webkit-linear-gradient(#ff5c28,#ff8f28);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  background-clip: text;
  display: inline-block;
  font-size: 16px;
}

.stats-logo {
  border-radius: 100%;
  width: 30px;
  height: 30px;
  margin-left: -20px;
  margin-right: 2px;
  max-height: 40px;
  max-width: 40px;
  outline: 1px solid rgba(255, 255, 255, 0.2);
}

.stats-value{
  color: #FEFEFE;
  font-size: 20px;
}

.separator-left {
  border-right: 1px solid #23242E;
}

#markets-container {
  background-color: #0F1018;
  border: 1px solid #23242E;
  font-size: 18px;
  height: 87%;
  padding: 0px 20px 0px 20px;
}

.market-row {
  align-items: center;
  display: flex;
  height: 16.66%; /* 100% / 6 */
}

.main-row{
  height: 92%; /* Market header is currently 10% */
}

/* Button */
.image-button {
  align-items: center;
  background-color: #F5F5F5;
  border: 0px solid #64676D;
  color: #0F1018;
  font-size: 13px;
  justify-content: center;
  margin: 5px 1px 1px 1px;
  padding: 4px 5px;
  width: 230px;
}

.image-button:hover {
  background-color: #F5F5F5;
  border: 0px;
  color: #353642;
}

.image-button:active {
  background-color: #F5F5F5;
  border: 0px;
  color: #23242E;
}

.image-button:active:focus {
  box-shadow: 0 0 0 0.1rem #F5F5F5;
}

.image-button:focus {
  background-color: #F5F5F5;
  border: 0px;
  box-shadow: none;
  color: #23242E;
}

.b-icon {
  margin-right: 6px;
  padding: 1px;
  vertical-align: text-top;
}

</style>
