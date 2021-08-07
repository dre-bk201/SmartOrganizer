<template>
  <div class="Option">
    <span>
      <slot name="header"></slot>
    </span>

    <div class="content">
      <slot name="content"></slot>
    </div>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss" scoped>
.Option {
  width: 100%;
  height: 90px;
  border-radius: 5px;
  background: $darkprimary;
  position: relative;
  box-sizing: border-box;
  box-shadow: $boxshadow;
  margin-bottom: 40px;
  padding: 25px 15px 15px 15px;

  span {
    background: $darkbg;
    padding: 8px 15px 8px 15px;
    color: getTextColor(dim);
    border-radius: 20px;
    position: absolute;
    top: -20px;
    left: 15px;
  }
}
</style>