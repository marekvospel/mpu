import { defineConfig } from 'vitepress'

// https://vitepress.vuejs.org/config/app-configs
export default defineConfig({
  base: '/mpu/',
  appearance: 'dark',
  title: 'MPU',
  lang: 'en',
  themeConfig: {
    socialLinks: [
      {
        icon: 'github',
        link: 'https://github.com/marekvospel/mpu'
      },
      {
        icon: 'mastodon',
        link: 'https://cyberplace.social/@vospel'
      }
    ],
    sidebar: [
      {
        text: 'Introduction',
        link: '/'
      },
      {
        text: 'ALU',
        link: '/alu/',
        items: [
          {
            text: 'ALU Operations',
            link: '/alu/operations'
          }
        ]
      }
    ],
    outline: [2, 3],
    algolia: {
      appId: '5LZ9QLZ9ZW',
      apiKey: '225bc788db3d8b1622885792ab6fd9c0',
      indexName: 'mpu'
    }
  },
})
