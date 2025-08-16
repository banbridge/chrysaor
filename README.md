## 代码实施

### 应用模块分层

核心原则为“高内聚，低耦合”，将模型和领域服务放在领域层，让仓储层和基础设施层倒置依赖领域层，从而为领域层服务。
参考[DDD](https://docs.mryqr.com/ddd-project-structure)，分包原则：先业务，后技术

#### 基本思想

- 按照基本职责来分层建设。
- 领域层是业务知识和业务规则的核心阵地，不过多关心技术细节的实现（如仓储、消息等），仓储层和基础设施层为其提供能力支撑。

#### 模块职责

##### 测试包（test）：

测试模块，所有单侧和集成测试写在该模块。它通过直接和间接依赖，可以访问到每个模块的代码，也即所有的模块对测试层都是可见的。

##### 主程序（main）：

应用的启动入口，一般是一个main.rs文件，使用[rudi](https://crates.io/crates/rudi)依赖注入框架来启动。

- command：用于存放应用服务以及命令对象等
- domain：用于存放所有领域模型
- eventhandler：用于存放领域事件处理器
- infrastructure：用于存放技术基础设施
- query：用于存放查询逻辑

#### 编码规范

__仅供参考__

##### POJO规范（强制）

POJO（Plain Ordinary Java Object）是指简单的Java对象，它没有任何业务逻辑，只用于数据传输。
在Rust中，仍沿用POJO的叫法，表示简单对象，POJO主要定义属性，很少定义方法，POJO队形后缀做如下约定。

###### PO 数据库模型

PO（Persistent Object）持久化对象，是指与数据库表对应的对象，它的属性与数据库表的字段一一对应。

###### DO 领域模型

DO（Domain Object）领域对象，XXDO是指领域层的对象，逻辑核心。

###### DTO 外部传输对象

DTO（Data Transfer Object）是对外数据传输对象，一般DTO在IDL中定义，然后通过工具自动生成，在HTTP或RPC服务中，DTO作为参数和返回值，用于数据传输。

###### Info 内部传输对象

XXInfo是在application和Domain层之间传递的对象，和DTO的功能类似，区别在于DTO对外，Info对内。

###### VO 值对象

VO（Value Object）值对象，XXVO是指可以接受其他系统facade传递过来的DTO。主义不是View Object（视图对象），项目中不需要视图对象。

###### Query 查询对象

XXQuery是仓储接口接收的查询参数封装对象，一般仓储查询包含3个及以上参数就应该封装。

###### Request请求对象

XXRequest是指IDL层定义的查询对象，一般一个facade接口3个及以上参数就应该封装。

###### Conv转换器

Conv（Converter）转换器，各种的类型对象涉及到大量的转换，Conv结尾的类就是转换器，XXAA2BBConv是将AA类型转换为BB类型的转换器。

##### Bean规范（强制）

Bean也是Java中的概念，Bean主要定义方法，很少定义属性，定义的属性一般也是对其他Bean的依赖。例如Service和Repository都是Bean。应用采用了多层架构，如果没有统一的命名规范，很容易导致不同层的Bean命名冲突。

###### Handler 处理器

对外提供http或者RPC服务的控制器，只存在于控制器层。

###### AppSvc 应用层服务

AppSvc（Application Service）应用层的服务层，以AppSvc结尾，可继续细化成XXWriteAppSvc操作服务和XXReadAppSvc查询服务。

###### DomainSvc 领域层服务

DomainSvc（Domain Service） 领域层服务，以DomainSvc结尾。

###### Repo 仓库接口

Repo（Repository） 仓库接口，以Repo结尾。

###### Client 中间件服务依赖接口

通过定义Client接口来消费中间件服务，Client定义在基础设施层。防腐设计

###### Caller 外部RPC服务依赖接口

通过定义Caller接口来消费外部RPC服务，Caller定义在领域层。防腐设计

###### Config 配置

Config（Configuration） 配置，以Configuration结尾，各层负责自己的配置，例如仓储层有数据库配置，在基础设施层有中间件的配置。

