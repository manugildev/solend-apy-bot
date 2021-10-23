<template>
    <b-container> 
        <b-col>
            <b-row class="supply_chart">
                <MarketChart title="Weekly Supply APY"
                             :chart_data="chart_data[0]"
                             :is_loading="is_loading"/>
            </b-row>
            <!-- Margin Row for Screenshot -->
            <b-row style="height: 10px;"></b-row>

            <b-row class="borrow_chart">
                <MarketChart title="Weekly Borrow APY"
                             :chart_data="chart_data[1]"
                             :is_loading="is_loading"/>
            </b-row>
        </b-col>
    </b-container> 
</template>

<script>
import Vue from "vue";
import MarketChart from "./MarketChart";

export default {
    name: "ChartContainer",
    components: { MarketChart },
    data() {
        return {
            chart_data: [],
            chart_data_cache: "",
            is_loading: true,
        };
    },
    mounted() {
        if (localStorage.chart_data_cache) { this.chart_data = JSON.parse(localStorage.chart_data_cache); }
    },
    watch: {
        chart_data_cache(new_data) { localStorage.chart_data_cache = new_data; },
    },
    async created() {
        this.is_loading = !Vue.config.devtools;
        if (Vue.config.devtools) return;

        // GET /chart_data request using fetch with async/await
        const response = await fetch("/chart_data");
        let chart_data = await response.json();
        this.chart_data_cache = JSON.stringify(chart_data);
        this.chart_data = chart_data;
        this.is_loading = false;
    },
}
</script>

<style>

</style>