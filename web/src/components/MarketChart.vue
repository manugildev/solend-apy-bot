<template>
    <b-aspect :class="class_name" class="chart" :aspect="'16:9'">
        <loading :active.sync="is_loading"
                    :is-full-page="full_page"
                    :loader="loader"
                    :opacity="opacity"
                    :background-color="backgroundColor"
                    :color="color"
                    :class="[is_loading ? 'not-loaded' : 'loaded']"></loading>
        <line-chart height="100%"
                    :data="chart_data"
                    :library="supply_library"
                    :key="component_key"
                    :colors="chart_line_colors"/>
                    <!-- :colors="chart_line_colors" -->
    </b-aspect>
</template>

<script>

import Vue from "vue";
import Loading from "vue-loading-overlay";
import "vue-loading-overlay/dist/vue-loading.css";
import Chartkick from "vue-chartkick";
import Chart from "chart.js";

Vue.use(Chartkick.use(Chart));
Chart.defaults.global.defaultFontColor = "white";
Chart.defaults.global.defaultFontFamily = "IBM Plex Sans";
Chart.defaults.global.defaultFontSize = 14;

export default {
    name: "MarketChart",
    components: { Loading },
    props: {
        title: String,
        index: Number,
        is_loading: Boolean,
        chart_data: Array,
        class_name: String,
    },
    data() {
      return {
        backgroundColor: "#0E1118",
        color: "#FF5C28",
        full_page: false,
        loader: "dots",
        opacity: 0.9,
        component_key: 0,
        chart_line_colors: [ "#469990", "#2775CA", "#F2F4F7", "#F58231", "#E6194B", "#3CB44B", "#DCBEFF", "#F032E6", "#FFE119", "#E74C3C", "#FABED4", "#42D4F4"],
        supply_library: {
            elements : {
                line: {
                    borderWidth: 0,
                },
                points: { radius: 2 }
            },
            title: {
                display: true,
                text: this.title,
                fontColor: "#FFF",
                fontFamily: "IBM Plex Sans",
                fontStyle: "normal",
            },
            layout: { padding: 15 },
            legend: { labels: { padding: 10, fontColor: "#fff", boxWidth: 14 } },
            scales: {
                yAxes: [
                    {
                        type: "linear",
                        position: "left",
                        autoSkip: false,
                        padding: 5,
                        gridLines: { color: '#353535' },
                        ticks: {
                            padding: 10,
                            maxTicksLimit: 8,
                            callback: function (value) { return value + "%"; },
                            beginAtZero: false,
                        },
                    },
                ],
                xAxes: [
                    {
                        type: "time",
                        distribution: "series",
                        padding: 10,
                        ticks: { padding: 8, maxTicksLimit: 7, maxRotation: 0, },
                        gridLines: { color: '#353536' },
                        time: { isoWeekday: true, unit: "day", },
                    },
                ],
            },
        },
      }
    },
    mounted() { this.forceRerender(); },
    watch: { is_loading() { this.forceRerender(); } },
    methods: {
        sleep(ms) {
            return new Promise(resolve => setTimeout(resolve, ms));
        },
        async forceRerender() {
            await this.sleep(300);
            this.component_key += 1;
        }
    }
}
</script>

<style>
.chart {
  background-color: #0f1018;
  border: 1px solid #23242e;
  padding-right: 10px;
}

</style>