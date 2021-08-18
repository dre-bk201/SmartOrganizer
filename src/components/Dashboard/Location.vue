<template>
  <div v-if="!advanced" class="Location">
    <div class="body">
      <img src="@/assets/images/folder-eye.svg" />
      <div class="content">
        <div>{{ dir }}</div>
        <span>{{ metadata }}</span>
      </div>
      <img
        :src="
          hover
            ? require('../../assets/images/remove-listener-r.svg')
            : require('../../assets/images/remove-listener.svg')
        "
        @click="remove(index, 'rules')"
        @mouseout="hover = false"
        @mouseenter="hover = true"
      />
    </div>
  </div>

  <div v-else class="ActionLocation">
    <div class="body">
      <img src="@/assets/images/destination.svg" />
      <div class="content">
        <div>{{ keyName }}</div>
        <span>{{ metadata }}</span>
      </div>
      <img
        :src="
          hover
            ? require('../../assets/images/remove-listener-r.svg')
            : require('../../assets/images/remove-listener.svg')
        "
        @click="remove(index, 'action')"
        @mouseleave.self="hover = false"
        @mouseenter.self="hover = true"
      />
    </div>
    <div class="option" @click="toggleExpand">
      <img src="../../assets/images/arrow.svg" />
    </div>
    <div class="expand">
      <div class="hr" />
      <div class="checkbox-options">
        <div class="checkbox-container">
          <input type="checkbox" id="FilenameCheckBoxCopy" value="COPY" />
          <label for="FilenameCheckBoxCopy" @click="handleCheckBoxClick($event)"
            >COPY</label
          >
        </div>

        <div class="checkbox-container">
          <input type="checkbox" id="FilenameCheckBoxMove" value="MOVE" />
          <label for="FilenameCheckBoxMove" @click="handleCheckBoxClick($event)"
            >MOVE</label
          >
        </div>

        <div class="checkbox-container">
          <input type="checkbox" id="FilenameCheckBoxDelete" value="DELETE" />
          <label
            for="FilenameCheckBoxDelete"
            @click="handleCheckBoxClick($event)"
            >DELETE</label
          >
        </div>

        <div class="checkbox-container">
          <input type="checkbox" id="FilenameCheckBoxUnlink" value="UNLINK" />
          <label
            for="FilenameCheckBoxUnlink"
            @click="handleCheckBoxClick($event)"
            >UNLINK</label
          >
        </div>

        <div class="checkbox-container">
          <input type="checkbox" id="FilenameCheckBoxNotify" value="NOTIFY" />
          <label
            for="FilenameCheckBoxNotify"
            @click="handleCheckBoxClick($event)"
            >NOTIFY</label
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import anime from "animejs";
import { $qsa, $qs } from "../../utils";
import { invoke } from "@tauri-apps/api";
export default {
  data: () => ({
    hover: false,
    isShown: false,
    selection: new Set(),
    metadata: "",
  }),
  props: {
    dir: String,
    index: Number,
    listener: Object,
    keyName: String,
    advanced: {
      type: Boolean,
      default: false,
    },
  },
  inject: ["remove", "updateAction"],
  methods: {
    toggleExpand() {
      const root = this.$el;
      const expanded = $qs(root, ".expand");
      const arrow = $qs(root, ".option img");
      const animation = {
        targets: arrow,
        duration: 200,
        rotate: "180deg",
        easing: "linear",
      };

      expanded.classList.toggle("show");
      root.classList.toggle("show");

      if (expanded.classList.contains("show")) {
        anime({ ...animation });
      } else {
        anime({ ...animation, rotate: "0deg" });
      }
    },

    validateSelection() {
      var possibleSelections = ["COPY", "MOVE", "DELETE", "UNLINK", "NOTIFY"];
      const removeConflicts = (conflicts) => {
        for (let i = 0; i < possibleSelections.length; i++)
          if (conflicts.includes(possibleSelections[i])) {
            possibleSelections.splice(i, 1);
            i -= 1;
          }
      };

      let selections = Array.from(this.selection);

      if (selections.includes("MOVE")) {
        removeConflicts(["DELETE", "UNLINK"]);
      } else if (selections.includes("DELETE"))
        removeConflicts(["MOVE", "UNLINK"]);
      else if (selections.includes("UNLINK"))
        removeConflicts(["MOVE", "DELETE"]);

      console.log("form: ", possibleSelections);
      $qsa(this.$el, ".checkbox-container").forEach((element) => {
        let actionType = $qs(element, "input").value;

        if (!possibleSelections.includes(actionType)) {
          element.classList.add("disabled");
          $qs(element, "input").disabled = true;
          this.selection.delete(actionType);
        } else {
          $qs(element, "input").disabled = false;
          element.classList.remove("disabled");
        }
      });
    },

    handleCheckBoxClick(event) {
      const checkbox = $qs(this.$el, "#" + event.target.htmlFor);
      if (!checkbox.disabled) {
        if (!checkbox.checked) this.selection.add(checkbox.value);
        else this.selection.delete(checkbox.value);
        this.validateSelection();

        this.updateAction(this.keyName, Array.from(this.selection));
      }
    },

    closeExpanded() {
      $qsa(document, ".ActionLocation.show").forEach((element) => {
        $qsa(element, ".expand").remove("show");
        element.classList.remove("show");
      });
    },

    normalizeText(text) {
      let data = text.toLowerCase();
      return data[0].toUpperCase() + data.slice(1);
    },
  },

  async mounted() {
    if (this.advanced) {
      let action_paths = this.listener.action_paths;
      let keys = Object.keys(action_paths);

      for (let i = 0; i < keys.length; i++)
        action_paths[keys[i]].forEach((element) => {
          const checkbox = $qs(
            this.$el,
            `#FilenameCheckBox${this.normalizeText(element)}`
          );

          checkbox.checked = true;
          this.selection.add(element);
          console.log("Ssa:", this.selection);
          this.validateSelection();
        });
    }

    this.metadata = `${await invoke("get_metadata", {
      path: this.dir + "*",
    })} files and folders`;
  },
};
</script>


<style lang="scss" scoped>
.disabled {
  opacity: 0.5;
}
.Location {
  border-radius: 8px;
  height: 58px;
  width: 100%;
  box-shadow: $boxshadow;
  background: $darkprimary;
  margin-bottom: 25px;

  .body {
    @include fullDimension();
    @include flexAlignCenter(row);
    justify-content: center;
    box-sizing: border-box;
    position: relative;

    .content {
      margin-left: 5px;
      box-sizing: border-box;
      width: calc(70%);
      color: white;

      div {
        width: calc(100%);
        display: inline-block;
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
        font-size: 13.5px;
      }

      span {
        font-size: 11.5px;
      }
    }

    img {
      height: 30px;
      user-select: none;
    }
  }
}

.ActionLocation {
  @extend .Location;
  position: relative;
  margin-bottom: 40px;
  transition: 0.3s;

  &.show {
    height: calc(58px + 90px);
  }

  .expand {
    background: $darkprimary;
    transition: 0.3s;
    border-radius: 10px;
    height: 0px;
    overflow: hidden;

    &.show {
      height: 90px;
    }

    .hr {
      height: 2px;
      background: rgb(78, 78, 78);
      box-shadow: 1px 1px 1px 0.2px rgba(0, 0, 0, 0.4);
    }

    .checkbox-options {
      height: calc(100%);
      width: 100%;
      padding: 15px;
      box-sizing: border-box;
      color: getTextColor(dim);
      display: flex;
      flex-flow: column wrap;

      .checkbox-container {
        @include alignCenter();
        transition: 0.3s;
        display: block;
        margin-bottom: 10px;
        margin-right: 30px;

        input {
          padding: 0;
          height: initial;
          width: initial;
          margin-bottom: 0;
          display: none;
          cursor: pointer;
        }

        label {
          position: relative;
          cursor: pointer;
          font-size: 14px;

          &:before {
            content: "";
            -webkit-appearance: none;
            background-color: transparent;
            border: 2px solid #888;
            border-radius: 5px;
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05),
              inset 0px -15px 10px -12px rgba(0, 0, 0, 0.05);
            padding: 7px;
            display: inline-block;
            position: relative;
            vertical-align: middle;
            cursor: pointer;
            margin-right: 5px;
          }
        }

        input + label:after {
          content: "";
          display: block;
          position: absolute;
          top: 0px;
          left: 9px;
          width: 0px;
          height: 0px;
          transform: rotate(45deg);
        }

        input:checked + label:after {
          content: "";
          display: block;
          position: absolute;
          top: 1px;
          left: 6px;
          border: solid $primary;
          border-width: 0 2px 2px 0;
          transform: rotate(45deg);
          animation: check 0.2s;
          animation-fill-mode: forwards;
        }

        @keyframes check {
          10% {
            opacity: 1;
          }
          40% {
            width: 3px;
          }
          50% {
            width: 5px;
          }
          60% {
            height: 4px;
          }
          100% {
            height: 9px;
            width: 5px;
          }
        }
      }
    }
  }

  .body {
    // position: absolute;
    // z-index: -999;
    height: 58px;
  }

  .option {
    position: absolute;
    background: $darkprimary;
    box-shadow: $boxshadow;
    right: 0px;
    bottom: -23px;
    width: 60px;
    height: 60px;
    z-index: -1;
    border-radius: 8px;
    display: flex;
    justify-content: center;

    img {
      width: 25px;
      position: absolute;
      bottom: 4px;
    }
  }
}
</style>