<template>
	<div class="item">
		<p title="Item">{{ detail.item }}</p>
		<p title="Precio">${{ detail.price }}</p>
		<p title="Cantidad">{{ detail.cant }}</p>
		<p title="Tipo">{{ detail.tipo }}</p>
		<p title="SubTotal">${{ detail.subtotal }}</p>
		<p title="IVA ($)">${{ detail.iva }}</p>
		<p title="Total">${{ detail.total }}</p>
		<button id="del-btn" @click="delDetail(index)" type="button">
			<svg-icon type="mdi" :path="mdiDelete"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiDelete } from '@mdi/js';

export default {
	setup() {
		return { mdiDelete }
	},
	components: {
	SvgIcon,
	},
	props: {
		detail: {
			type: Object,
			required: true
		}, 
		index: {
			type: Number,
			required: true,
		},
		delDetail: {
			type: Function,
			required: true,
		},
	},
};
</script>

<style scoped>
.item {
	width: 96%;
	display: grid;
	gap: 10px;
	grid-template-columns: 60px 88px 55px 120px 120px 88px 64px auto;
	justify-content: start;
	align-items: center;
}
.item > * {
	padding: 0 10px;
	margin: 2px 0 2px 0;
}

#del-btn {
	padding: 2px 6px 3px 6px;
	display: flex;
	align-items: center;
	justify-content: center;
	background: #333;
	color: #ccc;
	height: 2rem;
	width: 2rem;
	border: 1px solid #999;
	border-radius: .4rem;
	cursor: pointer;
	transition: background .2s;
}
#del-btn:hover {
	background: #543c3c;
}
</style>
