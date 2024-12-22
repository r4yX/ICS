<template>
  <div id="main">
		<div id="header">
			<h2>Trabajadores</h2>
		</div>
		<Worker />
  </div>
</template>

<script>
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount } from '@mdi/js';
import Worker from "../components/Worker.vue";

export default {
  name: 'Workers',
  components: {
    SvgIcon,
		Worker
  },
};
</script>

<style scoped>
#main {
  display: flex;
  flex-direction: column;
	background: #222;
	color: #ddd;
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 3rem;
}
</style>
