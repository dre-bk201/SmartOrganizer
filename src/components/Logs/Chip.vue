<template>
  <div class="Chip">
    <div class="title" :style="{ color }">
      {{ title }}
    </div>
    <div
      class="chip-content"
      @click="focus($event)"
      :style="{ background: bgColor, borderColor: color }"
    >
      <div class="dot" :style="{ background: color }" />
      <span :style="{ color }"> {{ name }} </span>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    name: {
      type: String,
      default: "MOVE",
    },
    title: String,
    color: {
      type: String,
      default: "#00F0FF",
    },
    bgColor: {
      type: String,
      default: "rgba(0, 1, 0, .20)",
    },
  },

  methods: {
    focus(e) {
      const chipContent = e.target.closest(".Chip");
      chipContent.classList.toggle("show-chip");
      chipContent.querySelector(".chip-content").classList.toggle("show-chip");
      console.log(chipContent);
    },
  },

  mounted() {},
};
</script>

<style lang="scss" scoped>
.Chip {
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: 0.3s;
  overflow: hidden;
  margin: 0px 5px 0px 5px;

  &.show-chip {
    width: 100%;
    position: absolute;
    backdrop-filter: blur(8px);
    display: flex;
    height: 105%;
    flex-direction: column;
    align-items: center;
  }

  .title {
    margin-left: 6px;
    font-size: 15px;
    margin-bottom: 10px;
  }

  .chip-content {
    @include flexAlignCenter(row);
    padding: 5px;
    transition: 0.3s;
    background: blue;
    box-sizing: border-box;
    border-radius: 10px;
    height: 25px;
    border: 1px solid purple;

    &:hover {
      cursor: pointer;
    }

    &.show-chip {
      backdrop-filter: blur(4px);
      transition: 0.3s;
      width: fit-content;
    }
    span {
      width: inherit;
      font-size: 12px;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }

    .dot {
      min-width: 7px;
      min-height: 7px;
      border-radius: 50%;
      margin-right: 6px;
    }
  }
}
</style>