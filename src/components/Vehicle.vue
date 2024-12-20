<template>
	<div class="item">
		<p title="Patente">{{ plate }}</p>
		<button id="del-btn" @click="delCar(index)" type="button">
			<svg-icon type="mdi" :path="mdiDelete"></svg-icon>
		</button>
	</div>
</template>

<script>
import { ref } from "vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiDelete } from '@mdi/js';

export default {
	components: { SvgIcon },
	setup() {
		return { mdiDelete }
	},
	props: {
		plate: {
			type: String,
			required: true
		}, 
		index: {
			type: Number,
			required: true,
		},
		delCar: {
			type: Function,
			required: true,
		},
	},
};
</script>

<style scoped>
.item {
	width: 100%;
	display: grid;
	gap: 10px;
	grid-template-columns: 64px 74px 63px 100px 130px 64px 54px auto;
	justify-content: start;
	align-items: center;
}
.item > * {
	padding: 0 10px;
}
#del-btn {
	padding: 2px 6px 3px 6px;
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2rem;
	width: 2rem;
	background: #333;
	cursor: pointer;
	border: 1px solid #999;
	border-radius: .4rem;
	color: white;
	margin-left: 1.4rem;
	transition: background .2s;
}
#del-btn:hover {
	background: #543c3c;
}
</style>
