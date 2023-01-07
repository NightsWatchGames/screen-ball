# screen-ball 在屏幕上踢球
- [ ] 足球运动模拟
- [ ] 鼠标控制踢球
- [ ] 3d模型载入
- [ ] 多显示器支持

## 参考资料
1. 概念来自 Bevy Discord 社区 PaulH#7052 发布的视频
[beach_ball.webm](https://user-images.githubusercontent.com/17514693/210358262-19bf32ef-b4f2-42a9-833e-4b9349816532.webm)
2. [bevy_rapier 文档](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy)

## 问题
**1.纹理（Texture）、贴图（Map）、材质（Material）怎么理解？**
> Texture纹理，就是一张图，一张PNG或者TAG等等格式的图片；
Map贴图，实际上是纹理贴图或者纹理映射（Texture Mapping），记住贴图不是图，是一种纹理映射技术，是三维模型的UV和纹理图片的对应关系；
>Material材质，是体现模型所有的可视属性，包含环境光、漫反射光、镜面反射光等等各种光照模型以及各种贴图等等。
> 在渲染三维模型的时候，需要对其赋予材质（Material），材质中除了调整各种光，还需要贴图（Texture Mapping）来表现更多细节，而贴图需要一张纹理图片。
> 摘自知乎评论 https://www.zhihu.com/question/25745472
> https://en.wikipedia.org/wiki/Texture_mapping

**2.踢足球运动模拟，是通过施加力Force还是冲量Impulse更好？**
