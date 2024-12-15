<template>
  <div id="main">
    <Menu class="menu" />
    <div class="content">
      <router-view />
    </div>
  </div>
</template>

<script>
import Menu from './components/Menu.vue';

export default {
  name: 'App',
  components: {
    Menu,
  },
};
</script>

<style scoped>
#main {
  display: flex;
	flex-direction: row; 
  height: 100vh; /* Asegura que todo ocupe el alto completo de la ventana */
}

.content {
  flex-grow: 1; /* Ocupa el espacio restante */
}
</style>
