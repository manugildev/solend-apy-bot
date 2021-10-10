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
import Loading from "vue-loading-overlay";
import "vue-loading-overlay/dist/vue-loading.css";

export default {
  name: "MarketData",
  components: { MarketHeader, MarketElement, Loading, },
  data() {
    return {
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
  },
  watch: {
    table_data_cache(new_data) { localStorage.table_data_cache = new_data; },
  },
  async created() {
    this.is_loading = !Vue.config.devtools;

    if (Vue.config.devtools) return;

    // GET /apy request using fetch with async/await
    let response = await fetch("/apy");
    let table_data = await response.json();
    this.table_data_cache = JSON.stringify(table_data);
    this.apys = table_data;

    this.is_loading = false;
  },

  methods: {
    /*         take_screenshot: function () {
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
#markets-container {
  background-color: #0f1018;
  height: 100%;
  font-size: 18px;
  padding: 0px 20px 0px 20px;
  border: 1px solid #23242e;
}

.market-row {
  height: 16.5%;
  display: flex;
  align-items: center;
}

/* Button */
.image-button {
  font-size: 13px;
  width: 230px;
  background-color: #f5f5f5;
  color: #0f1018;
  border: 0px solid #64676d;
  padding: 4px 5px;
  justify-content: center;
  align-items: center;
  margin: 5px 1px 1px 1px;
}

.image-button:hover {
  border: 0px;
  background-color: #f5f5f5;
  color: #353642;
}

.image-button:active {
  border: 0px;
  color: #23242e;
  background-color: #f5f5f5;
}

.image-button:active:focus {
  box-shadow: 0 0 0 0.1rem #f5f5f5;
}

.image-button:focus {
  border: 0px;
  color: #23242e;
  background-color: #f5f5f5;
  box-shadow: none;
}

.b-icon {
  vertical-align: text-top;
  margin-right: 6px;
  padding: 1px;
}

</style>
