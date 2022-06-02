<template>
    <div @click="this.newDir()" class="item-container">
            <folder-icon v-if="icon == 'folder'" :size="80" class="folder-icon"></folder-icon> 
            <file-icon v-else :size="80" ></file-icon>
        <div class="text-container">
            <p class="file-title">{{title ? title : "..."}}</p>
            <p @dblclick="this.getDate()" >{{this.date ? this.date : "..."}}</p>
        </div>
    </div>
</template>

<script>
import FolderIcon from 'vue-material-design-icons/Folder.vue';
import FileIcon from 'vue-material-design-icons/File.vue';

export default {
    data() {
        return {
            date : this.created,
        }
    },
    props : [
        'title',
        'created',
        'icon',
        'path'
        ],
    components : {
        FolderIcon,
        FileIcon
    },
    methods : {
        newDir() {
            if (this.icon == 'folder') {
                this.$parent.changePath(this.path);
            }
        },
        getDate(){
            window.__TAURI__.invoke('get_metadata', {'path' : this.path})
            .then(
                (data) => {
                    this.date = data;
                }
            );
        }
    },
    mounted() {
        this.getDate();
    },
}
</script>

<style scoped>
    .item-container{
        width : 230px;
        height: 100px;
        display: flex;
        border-radius: 5px;
        box-shadow: 0px 0px 10px rgb(139, 139, 139);
        justify-content: flex-start;
        align-items: center;
        margin: 10px;
        cursor: pointer;
        overflow: hidden;
        transition: all 0.5s ease-in-out;
    }

    .item-container:hover {
        box-shadow: 0px 0px 10px rgb(255, 102, 102);
    }

    .item-container:hover .material-design-icon__svg{ 
        fill:rgb(255, 102, 102) !important;
    }

    .folder-icon { 
        margin:10px;
    }

    .file-title{
        font-weight: 600;
        text-overflow: ellipsis;
        max-width: 82%;
        white-space: nowrap;
        overflow: hidden;
    }
    
    .text-container{
        max-width: 60%;
    }
</style>