<template>
  <div class="TreeBrowser">
    <div
      @click="expanded = !expanded"
      @dblclick="openFile"
      @mouseenter="showTooltip = true"
      @mouseleave="showTooltip = false"
      :style="{ 'margin-left': `${depth * 25}px` }"
      class="node"
    >
      <div class="icon">
        <!-- Renders Icon based on the type of file -->
        <img :src="getIcon" />
      </div>

      <!-- folder / filename -->
      <span class="node-name" :style="getStyle()">
        <div class="text">
          <!-- <Tooltip
            v-if="!hasChildren"
            :reveal="showTooltip"
            :nodeInformation="node"
          /> -->
          {{ node.name }}
          <span style="color: orange" v-if="!depth"> [ Root Dir ] </span>
        </div>
      </span>

      <span v-if="hasChildren" class="type">
        <div :class="'dropdown ' + `${expanded ? 'show' : 'hide'}`" />
      </span>
    </div>

    <div v-for="child in node.children" :key="child.name">
      <TreeBrowser
        v-if="expanded"
        :node="child"
        :depth="depth + 1"
        @expandNode="(node) => $emit('expandNode', node)"
      />
    </div>
  </div>
</template>

<script>
// import Tooltip from "../Tooltip/Tooltip";
// import { shell } from "electron";

export default {
  name: "TreeBrowser",
  props: {
    node: Object,
    depth: {
      type: Number,
      default: 0,
    },
  },
  components: {
    // Tooltip,
  },
  data: () => ({
    expanded: false,
    showTooltip: false,
    type: "",
  }),
  computed: {
    hasChildren() {
      if (this.node) {
        return this.node.children?.length;
      }
      return false;
    },
    getIcon() {
      const music = [".mp3", ".aac", ".wav", ".wma"];
      const video = [".mp4", ".avi", ".mkv", ".ts"];
      const image = [".jpg", ".jpeg", ".png", ".svg", ".gif", ".bmp"];
      console.log("node.name: ", this.node.name);
      const extension = this.extractExtension(this.node.name);
      console.log("ext: ", extension);
      if (this.hasChildren) {
        this.changeNodeType("folder");
        return require("../../assets/images/Folder.svg");
      } else {
        if (music.includes(extension)) {
          this.changeNodeType("music");
          return require("../../assets/images/Music.svg");
        } else if (video.includes(extension)) {
          this.changeNodeType("video");
          return require("../../assets/images/Video.svg");
        } else if (image.includes(extension)) {
          this.changeNodeType("image");
          return require("../../assets/images/Image.svg");
        } else {
          this.changeNodeType("file");
          return require("../../assets/images/Files.svg");
        }
      }
    },
  },
  methods: {
    isOpen() {},
    nodeClicked() {
      this.expanded = !this.expanded;
      if (!this.hasChildren) {
        this.$emit("expandNode", this.node);
      }
    },
    extractExtension(filename) {
      let returnValue = "";
      if (filename) {
        const split = filename.split(".");
        if (split.length > 1) returnValue = "." + split.slice(-1);
      }
      return returnValue;
    },
    changeNodeType(type) {
      this.type = type;
    },
    showDetails() {
      if (!this.hasChildren) {
        console.log(this.node.size);
      }
    },
    openFile() {
      if (!this.hasChildren) {
        // shell.openPath(this.node.path);
      }
    },
    getStyle() {
      let color = "#7432FF";
      if (this.type == "music") {
        color = "#F14668";
        return {
          color,
        };
      } else if (this.type == "video") {
        color = "#4BFF88";
        return {
          color,
        };
      } else if (this.type == "folder") {
        return {
          color,
        };
      } else if (this.type == "image") {
        color = "#FF32F7";
        return {
          color,
        };
      } else {
        color = "#FFAD4C";
        return {
          color,
        };
      }
    },
  },
  mounted() {
    if (!this.depth) this.expanded = true;
    setTimeout(() => {}); // opacity fade in animation
  },
};
</script>

<style scoped>
.TreeBrowser {
  position: relative;
}

.TreeBrowser:hover {
  cursor: pointer;
}

.node-name {
  position: relative;
  width: 85%;
  font-size: 13px;
}

.node-name .text {
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

img {
  width: 17px;
  height: 17px;
}

.node {
  text-align: left;
  font-size: 20px;
  margin-bottom: 9px;
  user-select: none;
  display: flex;
  align-items: center;
}

.dropdown {
  transition: 0.3s;
  display: inline-block;
  background: #7432ff;
  width: 12px;
  height: 8px;
  margin-left: 10px;
}

.dropdown.show {
  clip-path: polygon(50% 100%, 0 0, 100% 0);
}

.dropdown.hide {
  clip-path: polygon(50% 100%, 0 0, 100% 0);
  transform: rotate(-90deg);
}

.type {
  margin-right: 10px;
}

.icon {
  display: inline-block;
  margin-right: 10px;
}
</style>
