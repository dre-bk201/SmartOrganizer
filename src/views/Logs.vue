<template>
  <div id="Logs">
    <div class="search-container">
      <div class="search">
        <input type="text" v-model="search" placeholder="Searching for logs?" />
        <div class="search-icon">
          <img src="../assets/images/search.svg" />
        </div>
      </div>
    </div>

    <div class="logs-container">
      <!-- {{ $store.getters.getListeners }} -->
      <template v-for="listener in searchFilter" :key="listener.index">
        <LogView
          v-for="(log, index) in listener.logs"
          :log="log"
          :title="listener.title"
          :index="twoDKey([listener.index, index])"
          :key="twoDKey([listener.index, index])"
        />
      </template>
    </div>
  </div>
</template>

<script>
import LogView from "../components/Logs/LogView.vue";
import { mapGetters } from "vuex";
export default {
  name: "Logs",
  components: { LogView },
  data: () => ({ search: "" }),
  mounted() {
    // console.log("Mounted Logs");
  },
  methods: {
    range(length) {
      return [...Array(length).keys()];
    },
    twoDKey(dimensions) {
      return dimensions[0].toString() + dimensions[1].toString();
    },
  },
  computed: {
    ...mapGetters(["getListeners"]),
    searchFilter() {
      let _filter = this.getListeners;
      if (this.search)
        _filter = this.getListeners.filter((element) => {
          let search = this.search.toLowerCase();
          return element.title.toLowerCase().includes(search);
        });
      return _filter;
    },
  },
};
</script>

<style lang="scss" scoped>
#Logs {
  border-radius: 30px 0px 0px;
  background: $darkbg;
  height: 100%;
  width: calc(100vw - #{$nav-width} - #{$padding});
  position: absolute;
  top: 0px;
  left: $nav-width;
  box-sizing: border-box;
  padding: 0px 15px 0px 15px;

  .search-container {
    height: 70px;
    width: 100%;
    margin-bottom: 20px;
    display: grid;
    place-items: center;

    .search {
      // height: fit-content;
      // width: fit-content;
      position: relative;

      input {
        height: 38px;
        width: 60vw;
        border-radius: 3px;
        outline: none;
        text-indent: 10px;
        background: $darkprimary;
        color: white;
        font-family: Comfortaa;
        padding-right: 40px;
        border: none;
      }

      .search-icon {
        background: $primary;
        height: 38px;
        width: 40px;
        border-radius: 3px;
        position: absolute;
        display: grid;
        place-items: center;
        top: -0.5px;
        right: 0px;

        .search-icon img {
          height: 22px;
        }
      }
    }
  }

  .logs-container {
    padding: $padding * 1.5;
    box-sizing: border-box;
    height: calc(100% - 90px);
    width: 100%;
    overflow-y: auto;
    background: $darkprimary;
  }
}
</style>
