<template>
  <div id="main">
		<div id="header">
			<h2>Ordenes</h2>
		</div>
		<ul>
			<Budget
					v-for="(order, index) in orders"
					:key="index"
					:data="order"
					:index="index"
			/>
		</ul>
  </div>
</template>

<script>
import { ref, onMounted, shallowRef } from 'vue';                
import { invoke } from "@tauri-apps/api/core";
import Budget from "@components/Budget.vue";
import SvgIcon from '@jamescoyle/vue-icon';

export default {
  name: 'Works',
  components: {SvgIcon, Budget},
	setup() {
		const orders = ref([]);
		const updateOrders = async() => {
      try {
        const result = await invoke('obtain_orders');
        orders.value = result.map((order) => ({
					id: order.id,
          customer: order.customer,
          vehicle: order.vehicle,
          concept: order.concept,
          kilometrage: order.kilometrage,
          total: order.total,
					paid: order.paid,
					pay_date: "none",
        }));
      } catch (error) {
        console.error('Error al cargar ordenes:', error);
      }
    }
    onMounted(updateOrders);
		return {
			orders,
		};
	},
};
</script>

<style scoped>
#main {
  display: flex;
  flex-direction: column;
	background: #222;
	color: #ddd;
	overflow-y: scroll;
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 2rem;
}
</style>
