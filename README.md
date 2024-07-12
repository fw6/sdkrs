# 动态图片服务

提供类似[`imgix`](https://docs.imgix.com/apis/rendering)的图片服务，支持动态处理图片。

通过该服务提供的RESTful API，可以动态处理图片，这意味着只需调整图像`URL`的查询参数, 即可对其进行操作以创建新的图像转换。

例如原始图片地址`https://ak-d.tripcdn.com/images/1mi4k2215gl29og9c83A0.jpg`, 替换为`http://127.0.0.1:8080/dynamic-image/ak-d/1mi4k2215gl29og9c83A0.jpg?w=320&f=png`, 即可返回宽度为`320px`的`png`格式图片。

以上功能参考自: [Unsplash Developers | Dynamically resizable images](https://unsplash.com/documentation#dynamically-resizable-images)

> 当前图片来源仅限`tripcdn.com`域名

限制🚫:
- 限制最大尺寸`8192x8192`
- 支持编解码的图片格式, 如需其他格式请提`issue`
  - `jpeg`
  - `png`
  - `webp`
  - `gif`
  - `tiff`
  - `bmp`
  - `ico`

## 使用示例

```shell
$ curl -X 'GET' \
  'http://127.0.0.1:8080/dynamic-image/ak-d/1mi2812000do9ohfz825A.jpg?w=300&h=300&q=80&fm=png' \
  -H 'accept: application/octet-stream'
```

## Open API

```json
{
  "openapi": "3.0.3",
  "info": {
    "title": "sdkrs-openapi",
    "description": "",
    "contact": {
      "name": "feng.w",
      "email": "feng.w@trip.com"
    },
    "license": {
      "name": "Unlicense OR MIT"
    },
    "version": "0.0.1"
  },
  "paths": {
    "/dynamic": {
      "get": {
        "tags": [
          "dyn_image"
        ],
        "summary": "通过查询参数获取动态图片",
        "description": "通过查询参数, 拿到对应的图片资源后, 按照配置编辑图片并返回.",
        "operationId": "get_dyn_image",
        "parameters": [
          {
            "name": "width",
            "in": "query",
            "description": "图像宽度, 主模式是正整数, 如果传入`0.0`到`1.0`之间的值, 则会将其视为百分比\n\u003E 注意⚠️: 最大值为`8192`",
            "required": false,
            "schema": {
              "type": "number",
              "format": "float",
              "nullable": true
            }
          },
          {
            "name": "height",
            "in": "query",
            "description": "图像高度, 主模式是正整数, 如果传入`0.0`到`1.0`之间的值, 则会将其视为百分比\n\u003E 注意⚠️: 最大值为`8192`",
            "required": false,
            "schema": {
              "type": "number",
              "format": "float",
              "nullable": true
            }
          },
          {
            "name": "quality",
            "in": "query",
            "description": "在使用有损文件格式时(jpg, pjpg, webp, avif, or jxr)更改压缩质量. 有效值为`1`到`100`",
            "required": false,
            "schema": {
              "type": "integer",
              "format": "int32",
              "nullable": true,
              "minimum": 0
            }
          },
          {
            "name": "format",
            "in": "query",
            "description": "需要转换的图像格式, 传入的格式不支持时不转换",
            "required": false,
            "schema": {
              "type": "string",
              "nullable": true
            }
          },
          {
            "name": "crop",
            "in": "query",
            "description": "开启图像裁剪模式, 传入`crop`参数时必须传入`width`和`height`, 用于在特定图像尺寸内定义裁剪行为.\n有效的值为: `top`, `bottom`, `left`, `right`. 可以使用由`,`分割的多个值\n如果未设置, 则默认从图像中心开始裁剪",
            "required": false,
            "schema": {
              "type": "string",
              "nullable": true
            }
          },
          {
            "name": "fit",
            "in": "query",
            "description": "控制当导出的图像被缩放时, 背景该如何填充.\n有效的值为: `clamp`, `clip`, `crop`, `fill`, `scale`",
            "required": false,
            "schema": {
              "type": "string",
              "nullable": true
            }
          },
          {
            "name": "dpr",
            "in": "query",
            "description": "用于调整图像的设备像素比例, 宽高必须设置其中一个, 以便计算设备像素比.\n默认值是`1`, 最大值是`5`",
            "required": false,
            "schema": {
              "type": "integer",
              "format": "int32",
              "nullable": true,
              "minimum": 0
            }
          }
        ],
        "responses": {
          "200": {
            "description": "成功了🏅",
            "content": {
              "application/octet-stream": {
                "schema": {
                  "type": "string",
                  "format": "binary"
                }
              }
            }
          }
        }
      }
    }
  },
  "components": {
    "schemas": {
      "DynImageError": {
        "oneOf": [
          {
            "type": "object",
            "required": [
              "NotFound"
            ],
            "properties": {
              "NotFound": {
                "type": "string",
                "description": "Origin image not found."
              }
            },
            "example": "id = 1"
          },
          {
            "type": "object",
            "required": [
              "Unauthorized"
            ],
            "properties": {
              "Unauthorized": {
                "type": "string",
                "description": "Operation unauthorized"
              }
            },
            "example": "missing api key"
          }
        ],
        "description": "Dynamic image operation errors"
      }
    }
  }
}
```

## ROADMAP

- 开发/发布`rust`微服务Docker镜像
- 添加 `gitlab` CI/CD
- 移除背景图
- 面部识别, 以此为中心点进行裁剪
- 背景色填充
- 旋转
- 水印
- 支持更多格式
  - `svg`
  - `hdr`
- 提供命令行工具
- 提供`gRPC`版本服务
- 添加`benchmark`测试 (实现参考: [rust-web-benchmarks](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md))
