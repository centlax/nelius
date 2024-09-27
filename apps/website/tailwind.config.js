import { getIconCollections, iconsPlugin } from '@egoist/tailwindcss-icons';
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
      extend: {}
    },
    plugins: [
        iconsPlugin({
            collections: getIconCollections(['fluent']),
            scale: 1.5
        })
    ]
  };