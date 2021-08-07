module.exports = {
  css: {
    loaderOptions: {
      sass: {
        additionalData: `
            @import "@/assets/share/_colors.scss";
            @import "@/assets/share/_variables.scss";
            @import "@/assets/share/_mixins.scss";
          `,
      },
    },
  },
};
