<template>
  <div>
    <div id="file-container" v-for="File in this.currentFileList" :key="File.path">
      <item-object :title="File.name" :path="File.path" :icon="File.children ? 'folder' : 'file'" />
    </div>
    <!-- <button @click="this.getItems('C:\\Users\\craig\\Pictures\\VRChat')" >here</button> -->
    <button @click="this.getItems(this.currentPath)" >here</button>
    <button @click="this.changePath('C:\\')" >reset</button>
  </div>
</template>

<script>
import ItemObject from './components/Item-Object.vue';

export default {
  data() {
    return{
      currentFileList : [],
      currentPath : "C:\\",
      pathHistory : []
    }
  },
  components: { ItemObject },
  name: 'App',
  methods: {
    getItems(testFolder) {
      window.__TAURI__.fs.readDir(testFolder)
      .then(
        (Files) => {
          console.log(Files);
          this.currentFileList = Files;
        }
      );
    },
    changePath(path) {
      this.pathHistory.push(this.currentPath);
      this.currentPath = path;
      this.getItems(path);
    }
  },
}
</script>

<style>
  p {
    font-family: Arial, Helvetica, sans-serif;
  }

  #app > div{
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    align-content: center;
    justify-content: center;
    align-items: center;
  }
</style>