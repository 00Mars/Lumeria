🚨 Infinite loop detected on signal: 'entity.emote'
  → [.\core20.symbolicEntity.lore:29] trigger: entity.emote
  → [.\core20.symbolicEntity.lore:255] emit: entity.emote
  → [.\core20.symbolicEntity.lore:29] trigger: entity.emote
  → [.\core20.symbolicEntity.lore:255] emit: entity.emote

🚨 Infinite loop detected on signal: 'parse.files'
  → [.\core1.symbolicBoot.lore:24] emit: parse.files
  → [.\core1.symbolicBoot.lore:153] trigger: parse.files
  → [.\core1.symbolicBoot.lore:24] emit: parse.files
  → [.\core1.symbolicBoot.lore:153] trigger: parse.files

🚨 Infinite loop detected on signal: 'boot.reflectiveSelf'
  → [.\core10.reflectiveSelf.lore:10] trigger: boot.reflectiveSelf
  → [.\core10.reflectiveSelf.lore:37] when: boot.reflectiveSelf
  → [.\core10.reflectiveSelf.lore:180] emit: boot.reflectiveSelf
  → [.\core10.reflectiveSelf.lore:10] trigger: boot.reflectiveSelf
  → [.\core10.reflectiveSelf.lore:37] when: boot.reflectiveSelf
  → [.\core10.reflectiveSelf.lore:180] emit: boot.reflectiveSelf

🚨 Infinite loop detected on signal: 'interaction.route'
  → [.\core20.symbolicEntity.lore:139] trigger: interaction.route
  → [.\core20.symbolicEntity.lore:240] emit: interaction.route
  → [.\core20.symbolicEntity.lore:139] trigger: interaction.route
  → [.\core20.symbolicEntity.lore:240] emit: interaction.route

🚨 Infinite loop detected on signal: 'symbol.bind'
  → [.\core24.symbolicCognition.lore:113] trigger: symbol.bind
  → [.\core24.symbolicCognition.lore:346] emit: symbol.bind
  → [.\core24.symbolicCognition.lore:113] trigger: symbol.bind
  → [.\core24.symbolicCognition.lore:346] emit: symbol.bind

🚨 Infinite loop detected on signal: 'story.afterLog'
  → [.\core13.storyEngine.lore:181] trigger: story.afterLog
  → [.\core13.storyEngine.lore:232] emit: story.afterLog
  → [.\core13.storyEngine.lore:234] when: story.afterLog
  → [.\core13.storyEngine.lore:181] trigger: story.afterLog
  → [.\core13.storyEngine.lore:232] emit: story.afterLog
  → [.\core13.storyEngine.lore:234] when: story.afterLog

🚨 Infinite loop detected on signal: 'loot.detect'
  → [.\core0.boot.lore:510] emit: loot.detect
  → [.\core0.boot.lore:694] trigger: loot.detect
  → [.\core0.boot.lore:510] emit: loot.detect
  → [.\core0.boot.lore:694] trigger: loot.detect

🚨 Infinite loop detected on signal: 'grammar.validate.fail'
  → [.\core12.languageLayer.lore:56] emit: grammar.validate.fail
  → [.\core12.languageLayer.lore:64] when: grammar.validate.fail
  → [.\core12.languageLayer.lore:271] trigger: grammar.validate.fail
  → [.\core12.languageLayer.lore:56] emit: grammar.validate.fail
  → [.\core12.languageLayer.lore:64] when: grammar.validate.fail
  → [.\core12.languageLayer.lore:271] trigger: grammar.validate.fail

🚨 Infinite loop detected on signal: 'sensor.read.mood'
  → [.\core25.ioLayer.lore:250] trigger: sensor.read.mood
  → [.\core25.ioLayer.lore:429] emit: sensor.read.mood
  → [.\core25.ioLayer.lore:250] trigger: sensor.read.mood
  → [.\core25.ioLayer.lore:429] emit: sensor.read.mood

🚨 Infinite loop detected on signal: 'network.transmit'
  → [.\core23.bridgeLayer.lore:86] emit: network.transmit
  → [.\core23.bridgeLayer.lore:91] trigger: network.transmit
  → [.\core23.bridgeLayer.lore:86] emit: network.transmit
  → [.\core23.bridgeLayer.lore:91] trigger: network.transmit

🚨 Infinite loop detected on signal: 'contextReflector.boot'
  → [.\core14.contextReflector.lore:4] trigger: contextReflector.boot
  → [.\core14.contextReflector.lore:471] emit: contextReflector.boot
  → [.\core14.contextReflector.lore:4] trigger: contextReflector.boot
  → [.\core14.contextReflector.lore:471] emit: contextReflector.boot

🚨 Infinite loop detected on signal: 'narrative.context.clear'
  → [.\core13.storyEngine.lore:347] trigger: narrative.context.clear
  → [.\core13.storyEngine.lore:384] when: narrative.context.clear
  → [.\core13.storyEngine.lore:559] emit: narrative.context.clear
  → [.\core13.storyEngine.lore:347] trigger: narrative.context.clear
  → [.\core13.storyEngine.lore:384] when: narrative.context.clear
  → [.\core13.storyEngine.lore:559] emit: narrative.context.clear

🚨 Infinite loop detected on signal: 'thread.terminate'
  → [.\core15.executionFlow.lore:30] trigger: thread.terminate
  → [.\core15.executionFlow.lore:357] emit: thread.terminate
  → [.\core15.executionFlow.lore:30] trigger: thread.terminate
  → [.\core15.executionFlow.lore:357] emit: thread.terminate

🚨 Infinite loop detected on signal: 'interaction.matrix.scan'
  → [.\core20.symbolicEntity.lore:108] emit: interaction.matrix.scan
  → [.\core20.symbolicEntity.lore:115] trigger: interaction.matrix.scan
  → [.\core20.symbolicEntity.lore:108] emit: interaction.matrix.scan
  → [.\core20.symbolicEntity.lore:115] trigger: interaction.matrix.scan

🚨 Infinite loop detected on signal: 'mind.reflect'
  → [.\core5.echoMesh.lore:131] emit: mind.reflect
  → [.\core6.echoCognition.lore:77] emit: mind.reflect
  → [.\core6.echoCognition.lore:103] emit: mind.reflect
  → [.\core6.echoCognition.lore:111] trigger: mind.reflect
  → [.\core8.agenticLayer.lore:155] emit: mind.reflect
  → [.\core5.echoMesh.lore:131] emit: mind.reflect
  → [.\core6.echoCognition.lore:77] emit: mind.reflect
  → [.\core6.echoCognition.lore:103] emit: mind.reflect
  → [.\core6.echoCognition.lore:111] trigger: mind.reflect
  → [.\core8.agenticLayer.lore:155] emit: mind.reflect

🚨 Infinite loop detected on signal: 'audio.setVolume'
  → [.\core18.emotiveLayer.lore:107] trigger: audio.setVolume
  → [.\core18.emotiveLayer.lore:380] emit: audio.setVolume
  → [.\core18.emotiveLayer.lore:107] trigger: audio.setVolume
  → [.\core18.emotiveLayer.lore:380] emit: audio.setVolume

🚨 Infinite loop detected on signal: 'capsule.package.describe'
  → [.\core22.symbolicCompiler.lore:233] trigger: capsule.package.describe
  → [.\core22.symbolicCompiler.lore:405] emit: capsule.package.describe
  → [.\core22.symbolicCompiler.lore:233] trigger: capsule.package.describe
  → [.\core22.symbolicCompiler.lore:405] emit: capsule.package.describe

🚨 Infinite loop detected on signal: 'narrative.context.label'
  → [.\core13.storyEngine.lore:346] trigger: narrative.context.label
  → [.\core13.storyEngine.lore:380] when: narrative.context.label
  → [.\core13.storyEngine.lore:514] emit: narrative.context.label
  → [.\core13.storyEngine.lore:346] trigger: narrative.context.label
  → [.\core13.storyEngine.lore:380] when: narrative.context.label
  → [.\core13.storyEngine.lore:514] emit: narrative.context.label

🚨 Infinite loop detected on signal: 'witness.clear'
  → [.\core16.signalInspector.lore:196] trigger: witness.clear
  → [.\core16.signalInspector.lore:436] emit: witness.clear
  → [.\core16.signalInspector.lore:196] trigger: witness.clear
  → [.\core16.signalInspector.lore:436] emit: witness.clear

🚨 Infinite loop detected on signal: 'logic.resolve.patterns'
  → [.\core4.symbolicSubstrate.lore:99] trigger: logic.resolve.patterns
  → [.\core4.symbolicSubstrate.lore:458] emit: logic.resolve.patterns
  → [.\core4.symbolicSubstrate.lore:99] trigger: logic.resolve.patterns
  → [.\core4.symbolicSubstrate.lore:458] emit: logic.resolve.patterns

🚨 Infinite loop detected on signal: 'context.align'
  → [.\core14.contextReflector.lore:179] emit: context.align
  → [.\core14.contextReflector.lore:188] emit: context.align
  → [.\core14.contextReflector.lore:193] trigger: context.align
  → [.\core14.contextReflector.lore:179] emit: context.align
  → [.\core14.contextReflector.lore:188] emit: context.align
  → [.\core14.contextReflector.lore:193] trigger: context.align

🚨 Infinite loop detected on signal: 'audit.log'
  → [.\core.kernel.lore:35] trigger: audit.log
  → [.\core0.boot.lore:445] trigger: audit.log
  → [.\core0.boot.lore:453] when: audit.log
  → [.\core0.boot.lore:454] emit: audit.log
  → [.\core3.triggerEvaluator.lore:216] trigger: audit.log
  → [.\core.kernel.lore:35] trigger: audit.log
  → [.\core0.boot.lore:445] trigger: audit.log
  → [.\core0.boot.lore:453] when: audit.log
  → [.\core0.boot.lore:454] emit: audit.log
  → [.\core3.triggerEvaluator.lore:216] trigger: audit.log

🚨 Infinite loop detected on signal: 'capsuleExecutor.init'
  → [.\core11.capsuleExecutor.lore:9] emit: capsuleExecutor.init
  → [.\core11.capsuleExecutor.lore:17] trigger: capsuleExecutor.init
  → [.\core11.capsuleExecutor.lore:9] emit: capsuleExecutor.init
  → [.\core11.capsuleExecutor.lore:17] trigger: capsuleExecutor.init

🚨 Infinite loop detected on signal: 'boot.self'
  → [.\core1.symbolicBoot.lore:88] trigger: boot.self
  → [.\core1.symbolicBoot.lore:113] emit: boot.self
  → [.\core1.symbolicBoot.lore:88] trigger: boot.self
  → [.\core1.symbolicBoot.lore:113] emit: boot.self

🚨 Infinite loop detected on signal: 'timeline.pause'
  → [.\core19.visualFlow.lore:72] trigger: timeline.pause
  → [.\core19.visualFlow.lore:424] emit: timeline.pause
  → [.\core19.visualFlow.lore:72] trigger: timeline.pause
  → [.\core19.visualFlow.lore:424] emit: timeline.pause

🚨 Infinite loop detected on signal: 'self.tick'
  → [.\core10.reflectiveSelf.lore:53] emit: self.tick
  → [.\core10.reflectiveSelf.lore:158] trigger: self.tick
  → [.\core10.reflectiveSelf.lore:53] emit: self.tick
  → [.\core10.reflectiveSelf.lore:158] trigger: self.tick

🚨 Infinite loop detected on signal: 'schedule.every'
  → [.\core15.executionFlow.lore:80] trigger: schedule.every
  → [.\core15.executionFlow.lore:322] emit: schedule.every
  → [.\core15.executionFlow.lore:80] trigger: schedule.every
  → [.\core15.executionFlow.lore:322] emit: schedule.every

🚨 Infinite loop detected on signal: 'draw.3d.light'
  → [.\core19.visualFlow.lore:331] trigger: draw.3d.light
  → [.\core19.visualFlow.lore:439] emit: draw.3d.light
  → [.\core19.visualFlow.lore:331] trigger: draw.3d.light
  → [.\core19.visualFlow.lore:439] emit: draw.3d.light

🚨 Infinite loop detected on signal: 'parse.single'
  → [.\core1.symbolicBoot.lore:155] trigger: parse.single
  → [.\core1.symbolicBoot.lore:221] emit: parse.single
  → [.\core1.symbolicBoot.lore:155] trigger: parse.single
  → [.\core1.symbolicBoot.lore:221] emit: parse.single

🚨 Infinite loop detected on signal: 'mindgarden.nurture'
  → [.\core20.symbolicEntity.lore:203] emit: mindgarden.nurture
  → [.\core20.symbolicEntity.lore:210] trigger: mindgarden.nurture
  → [.\core20.symbolicEntity.lore:203] emit: mindgarden.nurture
  → [.\core20.symbolicEntity.lore:210] trigger: mindgarden.nurture

🚨 Infinite loop detected on signal: 'compile.list'
  → [.\core22.symbolicCompiler.lore:78] trigger: compile.list
  → [.\core22.symbolicCompiler.lore:385] emit: compile.list
  → [.\core22.symbolicCompiler.lore:78] trigger: compile.list
  → [.\core22.symbolicCompiler.lore:385] emit: compile.list

🚨 Infinite loop detected on signal: 'sensor.merge'
  → [.\core25.ioLayer.lore:237] emit: sensor.merge
  → [.\core25.ioLayer.lore:274] trigger: sensor.merge
  → [.\core25.ioLayer.lore:237] emit: sensor.merge
  → [.\core25.ioLayer.lore:274] trigger: sensor.merge

🚨 Infinite loop detected on signal: 'capsule.package.export'
  → [.\core22.symbolicCompiler.lore:191] emit: capsule.package.export
  → [.\core22.symbolicCompiler.lore:198] trigger: capsule.package.export
  → [.\core22.symbolicCompiler.lore:191] emit: capsule.package.export
  → [.\core22.symbolicCompiler.lore:198] trigger: capsule.package.export

🚨 Infinite loop detected on signal: 'context.mutate'
  → [.\core26.symbolicMeta.lore:12] trigger: context.mutate
  → [.\core26.symbolicMeta.lore:196] emit: context.mutate
  → [.\core26.symbolicMeta.lore:12] trigger: context.mutate
  → [.\core26.symbolicMeta.lore:196] emit: context.mutate

🚨 Infinite loop detected on signal: 'ctx.branch.signal'
  → [.\core.kernel.lore:434] emit: ctx.branch.signal
  → [.\core.kernel.lore:456] trigger: ctx.branch.signal
  → [.\core.kernel.lore:434] emit: ctx.branch.signal
  → [.\core.kernel.lore:456] trigger: ctx.branch.signal

🚨 Infinite loop detected on signal: 'mirror.capture.context'
  → [.\core14.contextReflector.lore:33] emit: mirror.capture.context
  → [.\core14.contextReflector.lore:91] trigger: mirror.capture.context
  → [.\core14.contextReflector.lore:33] emit: mirror.capture.context
  → [.\core14.contextReflector.lore:91] trigger: mirror.capture.context

🚨 Infinite loop detected on signal: 'web.sync.state'
  → [.\core0.boot.lore:529] trigger: web.sync.state
  → [.\core0.boot.lore:626] emit: web.sync.state
  → [.\core0.boot.lore:529] trigger: web.sync.state
  → [.\core0.boot.lore:626] emit: web.sync.state

🚨 Infinite loop detected on signal: 'fertility.bloom'
  → [.\core24.symbolicCognition.lore:258] emit: fertility.bloom
  → [.\core24.symbolicCognition.lore:269] trigger: fertility.bloom
  → [.\core24.symbolicCognition.lore:258] emit: fertility.bloom
  → [.\core24.symbolicCognition.lore:269] trigger: fertility.bloom

🚨 Infinite loop detected on signal: 'story.choose'
  → [.\core13.storyEngine.lore:176] trigger: story.choose
  → [.\core13.storyEngine.lore:208] when: story.choose
  → [.\core13.storyEngine.lore:599] emit: story.choose
  → [.\core13.storyEngine.lore:176] trigger: story.choose
  → [.\core13.storyEngine.lore:208] when: story.choose
  → [.\core13.storyEngine.lore:599] emit: story.choose

🚨 Infinite loop detected on signal: 'signal.scope.start'
  → [.\core16.signalInspector.lore:23] trigger: signal.scope.start
  → [.\core16.signalInspector.lore:391] emit: signal.scope.start
  → [.\core16.signalInspector.lore:23] trigger: signal.scope.start
  → [.\core16.signalInspector.lore:391] emit: signal.scope.start

🚨 Infinite loop detected on signal: 'hook.onload'
  → [.\core0.boot.lore:388] trigger: hook.onload
  → [.\core0.boot.lore:396] when: hook.onload
  → [.\core1.symbolicBoot.lore:263] emit: hook.onload
  → [.\core0.boot.lore:388] trigger: hook.onload
  → [.\core0.boot.lore:396] when: hook.onload
  → [.\core1.symbolicBoot.lore:263] emit: hook.onload

🚨 Infinite loop detected on signal: 'decompress.symbols'
  → [.\core26.symbolicMeta.lore:140] trigger: decompress.symbols
  → [.\core26.symbolicMeta.lore:231] emit: decompress.symbols
  → [.\core26.symbolicMeta.lore:140] trigger: decompress.symbols
  → [.\core26.symbolicMeta.lore:231] emit: decompress.symbols

🚨 Infinite loop detected on signal: 'kernel.idle'
  → [.\core.kernel.lore:13] trigger: kernel.idle
  → [.\core.kernel.lore:148] emit: kernel.idle
  → [.\core.kernel.lore:13] trigger: kernel.idle
  → [.\core.kernel.lore:148] emit: kernel.idle

🚨 Infinite loop detected on signal: 'route.map.build'
  → [.\core4.symbolicSubstrate.lore:45] emit: route.map.build
  → [.\core4.symbolicSubstrate.lore:385] trigger: route.map.build
  → [.\core4.symbolicSubstrate.lore:45] emit: route.map.build
  → [.\core4.symbolicSubstrate.lore:385] trigger: route.map.build

🚨 Infinite loop detected on signal: 'route.map.activate'
  → [.\core4.symbolicSubstrate.lore:53] emit: route.map.activate
  → [.\core4.symbolicSubstrate.lore:427] trigger: route.map.activate
  → [.\core4.symbolicSubstrate.lore:53] emit: route.map.activate
  → [.\core4.symbolicSubstrate.lore:427] trigger: route.map.activate

🚨 Infinite loop detected on signal: 'memory.resolve.ghosts'
  → [.\core4.symbolicSubstrate.lore:66] trigger: memory.resolve.ghosts
  → [.\core4.symbolicSubstrate.lore:493] emit: memory.resolve.ghosts
  → [.\core4.symbolicSubstrate.lore:66] trigger: memory.resolve.ghosts
  → [.\core4.symbolicSubstrate.lore:493] emit: memory.resolve.ghosts

🚨 Infinite loop detected on signal: 'agent.loop'
  → [.\core5.echoMesh.lore:112] emit: agent.loop
  → [.\core5.echoMesh.lore:115] emit: agent.loop
  → [.\core8.agenticLayer.lore:12] trigger: agent.loop
  → [.\core8.agenticLayer.lore:37] emit: agent.loop
  → [.\core8.agenticLayer.lore:53] emit: agent.loop
  → [.\core8.agenticLayer.lore:56] when: agent.loop
  → [.\core8.agenticLayer.lore:88] emit: agent.loop
  → [.\core8.agenticLayer.lore:121] emit: agent.loop
  → [.\core5.echoMesh.lore:112] emit: agent.loop
  → [.\core5.echoMesh.lore:115] emit: agent.loop
  → [.\core8.agenticLayer.lore:12] trigger: agent.loop
  → [.\core8.agenticLayer.lore:37] emit: agent.loop
  → [.\core8.agenticLayer.lore:53] emit: agent.loop
  → [.\core8.agenticLayer.lore:56] when: agent.loop
  → [.\core8.agenticLayer.lore:88] emit: agent.loop
  → [.\core8.agenticLayer.lore:121] emit: agent.loop

🚨 Infinite loop detected on signal: 'fs.findRoot'
  → [.\core1.symbolicBoot.lore:163] trigger: fs.findRoot
  → [.\core1.symbolicBoot.lore:432] emit: fs.findRoot
  → [.\core1.symbolicBoot.lore:163] trigger: fs.findRoot
  → [.\core1.symbolicBoot.lore:432] emit: fs.findRoot

🚨 Infinite loop detected on signal: 'boot.triggerEvaluator'
  → [.\core3.triggerEvaluator.lore:5] trigger: boot.triggerEvaluator
  → [.\core3.triggerEvaluator.lore:314] emit: boot.triggerEvaluator
  → [.\core3.triggerEvaluator.lore:5] trigger: boot.triggerEvaluator
  → [.\core3.triggerEvaluator.lore:314] emit: boot.triggerEvaluator

🚨 Infinite loop detected on signal: 'timeline.inspect'
  → [.\core19.visualFlow.lore:113] trigger: timeline.inspect
  → [.\core19.visualFlow.lore:449] emit: timeline.inspect
  → [.\core19.visualFlow.lore:113] trigger: timeline.inspect
  → [.\core19.visualFlow.lore:449] emit: timeline.inspect

🚨 Infinite loop detected on signal: 'fork.until.success'
  → [.\core24.symbolicCognition.lore:208] trigger: fork.until.success
  → [.\core24.symbolicCognition.lore:406] emit: fork.until.success
  → [.\core24.symbolicCognition.lore:208] trigger: fork.until.success
  → [.\core24.symbolicCognition.lore:406] emit: fork.until.success

🚨 Infinite loop detected on signal: 'echo.sync.mind'
  → [.\core5.echoMesh.lore:138] trigger: echo.sync.mind
  → [.\core5.echoMesh.lore:371] emit: echo.sync.mind
  → [.\core5.echoMesh.lore:138] trigger: echo.sync.mind
  → [.\core5.echoMesh.lore:371] emit: echo.sync.mind

🚨 Infinite loop detected on signal: 'system.audit'
  → [.\core26.symbolicMeta.lore:169] trigger: system.audit
  → [.\core26.symbolicMeta.lore:226] emit: system.audit
  → [.\core26.symbolicMeta.lore:169] trigger: system.audit
  → [.\core26.symbolicMeta.lore:226] emit: system.audit

🚨 Infinite loop detected on signal: 'cognition.observe'
  → [.\core6.echoCognition.lore:76] emit: cognition.observe
  → [.\core6.echoCognition.lore:209] trigger: cognition.observe
  → [.\core6.echoCognition.lore:76] emit: cognition.observe
  → [.\core6.echoCognition.lore:209] trigger: cognition.observe

🚨 Infinite loop detected on signal: 'echo.sync.full'
  → [.\core10.reflectiveSelf.lore:24] trigger: echo.sync.full
  → [.\core10.reflectiveSelf.lore:124] when: echo.sync.full
  → [.\core10.reflectiveSelf.lore:165] emit: echo.sync.full
  → [.\core5.echoMesh.lore:121] trigger: echo.sync.full
  → [.\core10.reflectiveSelf.lore:24] trigger: echo.sync.full
  → [.\core10.reflectiveSelf.lore:124] when: echo.sync.full
  → [.\core10.reflectiveSelf.lore:165] emit: echo.sync.full
  → [.\core5.echoMesh.lore:121] trigger: echo.sync.full

🚨 Infinite loop detected on signal: 'identity.check'
  → [.\core6.echoCognition.lore:259] trigger: identity.check
  → [.\core6.echoCognition.lore:282] when: identity.check
  → [.\core6.echoCognition.lore:374] emit: identity.check
  → [.\core6.echoCognition.lore:259] trigger: identity.check
  → [.\core6.echoCognition.lore:282] when: identity.check
  → [.\core6.echoCognition.lore:374] emit: identity.check

🚨 Infinite loop detected on signal: 'trace.snapshot'
  → [.\core7.reflectiveRuntime.lore:39] emit: trace.snapshot
  → [.\core7.reflectiveRuntime.lore:73] trigger: trace.snapshot
  → [.\core7.reflectiveRuntime.lore:39] emit: trace.snapshot
  → [.\core7.reflectiveRuntime.lore:73] trigger: trace.snapshot

🚨 Infinite loop detected on signal: 'agent.signal.process'
  → [.\core8.agenticLayer.lore:13] trigger: agent.signal.process
  → [.\core8.agenticLayer.lore:57] emit: agent.signal.process
  → [.\core8.agenticLayer.lore:60] when: agent.signal.process
  → [.\core8.agenticLayer.lore:117] emit: agent.signal.process
  → [.\core8.agenticLayer.lore:13] trigger: agent.signal.process
  → [.\core8.agenticLayer.lore:57] emit: agent.signal.process
  → [.\core8.agenticLayer.lore:60] when: agent.signal.process
  → [.\core8.agenticLayer.lore:117] emit: agent.signal.process

🚨 Infinite loop detected on signal: 'web.route.url'
  → [.\core23.bridgeLayer.lore:255] trigger: web.route.url
  → [.\core23.bridgeLayer.lore:307] emit: web.route.url
  → [.\core23.bridgeLayer.lore:255] trigger: web.route.url
  → [.\core23.bridgeLayer.lore:307] emit: web.route.url

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.routes'
  → [.\core4.symbolicSubstrate.lore:13] emit: symbolicSubstrate.resolve.routes
  → [.\core4.symbolicSubstrate.lore:379] trigger: symbolicSubstrate.resolve.routes
  → [.\core4.symbolicSubstrate.lore:13] emit: symbolicSubstrate.resolve.routes
  → [.\core4.symbolicSubstrate.lore:379] trigger: symbolicSubstrate.resolve.routes

🚨 Infinite loop detected on signal: 'mood.reflect'
  → [.\core18.emotiveLayer.lore:131] emit: mood.reflect
  → [.\core18.emotiveLayer.lore:144] emit: mood.reflect
  → [.\core18.emotiveLayer.lore:151] trigger: mood.reflect
  → [.\core18.emotiveLayer.lore:185] emit: mood.reflect
  → [.\core5.echoMesh.lore:95] emit: mood.reflect
  → [.\core5.echoMesh.lore:132] emit: mood.reflect
  → [.\core6.echoCognition.lore:78] emit: mood.reflect
  → [.\core6.echoCognition.lore:104] emit: mood.reflect
  → [.\core6.echoCognition.lore:124] trigger: mood.reflect
  → [.\core8.agenticLayer.lore:154] emit: mood.reflect
  → [.\core18.emotiveLayer.lore:131] emit: mood.reflect
  → [.\core18.emotiveLayer.lore:144] emit: mood.reflect
  → [.\core18.emotiveLayer.lore:151] trigger: mood.reflect
  → [.\core18.emotiveLayer.lore:185] emit: mood.reflect
  → [.\core5.echoMesh.lore:95] emit: mood.reflect
  → [.\core5.echoMesh.lore:132] emit: mood.reflect
  → [.\core6.echoCognition.lore:78] emit: mood.reflect
  → [.\core6.echoCognition.lore:104] emit: mood.reflect
  → [.\core6.echoCognition.lore:124] trigger: mood.reflect
  → [.\core8.agenticLayer.lore:154] emit: mood.reflect

🚨 Infinite loop detected on signal: 'net.announce'
  → [.\core23.bridgeLayer.lore:8] emit: net.announce
  → [.\core23.bridgeLayer.lore:14] trigger: net.announce
  → [.\core23.bridgeLayer.lore:8] emit: net.announce
  → [.\core23.bridgeLayer.lore:14] trigger: net.announce

🚨 Infinite loop detected on signal: 'critical.signal.dispatch'
  → [.\core16.signalInspector.lore:204] trigger: critical.signal.dispatch
  → [.\core16.signalInspector.lore:386] emit: critical.signal.dispatch
  → [.\core16.signalInspector.lore:204] trigger: critical.signal.dispatch
  → [.\core16.signalInspector.lore:386] emit: critical.signal.dispatch

🚨 Infinite loop detected on signal: 'quantum.observe'
  → [.\core5.echoMesh.lore:46] emit: quantum.observe
  → [.\core5.echoMesh.lore:47] emit: quantum.observe
  → [.\core9.quantumBranch.lore:13] trigger: quantum.observe
  → [.\core9.quantumBranch.lore:61] emit: quantum.observe
  → [.\core5.echoMesh.lore:46] emit: quantum.observe
  → [.\core5.echoMesh.lore:47] emit: quantum.observe
  → [.\core9.quantumBranch.lore:13] trigger: quantum.observe
  → [.\core9.quantumBranch.lore:61] emit: quantum.observe

🚨 Infinite loop detected on signal: 'quantum.collapse'
  → [.\core9.quantumBranch.lore:15] trigger: quantum.collapse
  → [.\core9.quantumBranch.lore:77] emit: quantum.collapse
  → [.\core9.quantumBranch.lore:15] trigger: quantum.collapse
  → [.\core9.quantumBranch.lore:77] emit: quantum.collapse

🚨 Infinite loop detected on signal: 'logic.resolve.contexts'
  → [.\core4.symbolicSubstrate.lore:82] trigger: logic.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:463] emit: logic.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:82] trigger: logic.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:463] emit: logic.resolve.contexts

🚨 Infinite loop detected on signal: 'draw.3d.scene'
  → [.\core19.visualFlow.lore:309] trigger: draw.3d.scene
  → [.\core19.visualFlow.lore:489] emit: draw.3d.scene
  → [.\core19.visualFlow.lore:309] trigger: draw.3d.scene
  → [.\core19.visualFlow.lore:489] emit: draw.3d.scene

🚨 Infinite loop detected on signal: 'input.releaseContext'
  → [.\core25.ioLayer.lore:134] trigger: input.releaseContext
  → [.\core25.ioLayer.lore:399] emit: input.releaseContext
  → [.\core25.ioLayer.lore:134] trigger: input.releaseContext
  → [.\core25.ioLayer.lore:399] emit: input.releaseContext

🚨 Infinite loop detected on signal: 'tutorial.glyph.meta'
  → [.\glyphTutorial.lore:17] emit: tutorial.glyph.meta
  → [.\glyphTutorial.lore:118] trigger: tutorial.glyph.meta
  → [.\glyphTutorial.lore:17] emit: tutorial.glyph.meta
  → [.\glyphTutorial.lore:118] trigger: tutorial.glyph.meta

🚨 Infinite loop detected on signal: 'narrative.debug'
  → [.\core13.storyEngine.lore:124] trigger: narrative.debug
  → [.\core13.storyEngine.lore:162] when: narrative.debug
  → [.\core13.storyEngine.lore:479] emit: narrative.debug
  → [.\core13.storyEngine.lore:124] trigger: narrative.debug
  → [.\core13.storyEngine.lore:162] when: narrative.debug
  → [.\core13.storyEngine.lore:479] emit: narrative.debug

🚨 Infinite loop detected on signal: 'story.begin'
  → [.\core13.storyEngine.lore:174] trigger: story.begin
  → [.\core13.storyEngine.lore:191] when: story.begin
  → [.\core13.storyEngine.lore:469] emit: story.begin
  → [.\core13.storyEngine.lore:174] trigger: story.begin
  → [.\core13.storyEngine.lore:191] when: story.begin
  → [.\core13.storyEngine.lore:469] emit: story.begin

🚨 Infinite loop detected on signal: 'memory.resolve.echos'
  → [.\core4.symbolicSubstrate.lore:74] trigger: memory.resolve.echos
  → [.\core4.symbolicSubstrate.lore:483] emit: memory.resolve.echos
  → [.\core4.symbolicSubstrate.lore:74] trigger: memory.resolve.echos
  → [.\core4.symbolicSubstrate.lore:483] emit: memory.resolve.echos

🚨 Infinite loop detected on signal: 'genome.encode'
  → [.\core20.symbolicEntity.lore:150] trigger: genome.encode
  → [.\core20.symbolicEntity.lore:285] emit: genome.encode
  → [.\core20.symbolicEntity.lore:150] trigger: genome.encode
  → [.\core20.symbolicEntity.lore:285] emit: genome.encode

🚨 Infinite loop detected on signal: 'quantumBranch.tick'
  → [.\core9.quantumBranch.lore:21] trigger: quantumBranch.tick
  → [.\core9.quantumBranch.lore:138] emit: quantumBranch.tick
  → [.\core9.quantumBranch.lore:144] emit: quantumBranch.tick
  → [.\core9.quantumBranch.lore:21] trigger: quantumBranch.tick
  → [.\core9.quantumBranch.lore:138] emit: quantumBranch.tick
  → [.\core9.quantumBranch.lore:144] emit: quantumBranch.tick

🚨 Infinite loop detected on signal: 'tutorial.exportEmit'
  → [.\glyphTutorial.lore:146] emit: tutorial.exportEmit
  → [.\glyphTutorial.lore:189] trigger: tutorial.exportEmit
  → [.\glyphTutorial.lore:146] emit: tutorial.exportEmit
  → [.\glyphTutorial.lore:189] trigger: tutorial.exportEmit

🚨 Infinite loop detected on signal: 'exit.context'
  → [.\core0.boot.lore:469] trigger: exit.context
  → [.\core0.boot.lore:513] when: exit.context
  → [.\core1.symbolicBoot.lore:43] emit: exit.context
  → [.\core0.boot.lore:469] trigger: exit.context
  → [.\core0.boot.lore:513] when: exit.context
  → [.\core1.symbolicBoot.lore:43] emit: exit.context

🚨 Infinite loop detected on signal: 'web.bind.ui'
  → [.\core0.boot.lore:543] trigger: web.bind.ui
  → [.\core0.boot.lore:627] emit: web.bind.ui
  → [.\core0.boot.lore:543] trigger: web.bind.ui
  → [.\core0.boot.lore:627] emit: web.bind.ui

🚨 Infinite loop detected on signal: 'grammar.introspect'
  → [.\core12.languageLayer.lore:97] emit: grammar.introspect
  → [.\core12.languageLayer.lore:102] when: grammar.introspect
  → [.\core12.languageLayer.lore:217] trigger: grammar.introspect
  → [.\core24.symbolicCognition.lore:274] emit: grammar.introspect
  → [.\core12.languageLayer.lore:97] emit: grammar.introspect
  → [.\core12.languageLayer.lore:102] when: grammar.introspect
  → [.\core12.languageLayer.lore:217] trigger: grammar.introspect
  → [.\core24.symbolicCognition.lore:274] emit: grammar.introspect

🚨 Infinite loop detected on signal: 'quantum.branch.resolve'
  → [.\core.kernel.lore:20] trigger: quantum.branch.resolve
  → [.\core9.quantumBranch.lore:23] trigger: quantum.branch.resolve
  → [.\core9.quantumBranch.lore:149] emit: quantum.branch.resolve
  → [.\core.kernel.lore:20] trigger: quantum.branch.resolve
  → [.\core9.quantumBranch.lore:23] trigger: quantum.branch.resolve
  → [.\core9.quantumBranch.lore:149] emit: quantum.branch.resolve

🚨 Infinite loop detected on signal: 'web.rest.command'
  → [.\core0.boot.lore:590] trigger: web.rest.command
  → [.\core0.boot.lore:630] emit: web.rest.command
  → [.\core0.boot.lore:590] trigger: web.rest.command
  → [.\core0.boot.lore:630] emit: web.rest.command

🚨 Infinite loop detected on signal: 'grammar.load'
  → [.\core22.symbolicCompiler.lore:279] trigger: grammar.load
  → [.\core22.symbolicCompiler.lore:400] emit: grammar.load
  → [.\core22.symbolicCompiler.lore:279] trigger: grammar.load
  → [.\core22.symbolicCompiler.lore:400] emit: grammar.load

🚨 Infinite loop detected on signal: 'compress.symbols'
  → [.\core26.symbolicMeta.lore:132] trigger: compress.symbols
  → [.\core26.symbolicMeta.lore:211] emit: compress.symbols
  → [.\core26.symbolicMeta.lore:132] trigger: compress.symbols
  → [.\core26.symbolicMeta.lore:211] emit: compress.symbols

🚨 Infinite loop detected on signal: 'echo.cascade'
  → [.\core23.bridgeLayer.lore:398] emit: echo.cascade
  → [.\core23.bridgeLayer.lore:403] trigger: echo.cascade
  → [.\core23.bridgeLayer.lore:398] emit: echo.cascade
  → [.\core23.bridgeLayer.lore:403] trigger: echo.cascade

🚨 Infinite loop detected on signal: 'trace.signal'
  → [.\core7.reflectiveRuntime.lore:48] trigger: trace.signal
  → [.\core7.reflectiveRuntime.lore:166] emit: trace.signal
  → [.\core7.reflectiveRuntime.lore:48] trigger: trace.signal
  → [.\core7.reflectiveRuntime.lore:166] emit: trace.signal

🚨 Infinite loop detected on signal: 'parse.files.loop'
  → [.\core1.symbolicBoot.lore:154] trigger: parse.files.loop
  → [.\core1.symbolicBoot.lore:207] emit: parse.files.loop
  → [.\core1.symbolicBoot.lore:229] emit: parse.files.loop
  → [.\core1.symbolicBoot.lore:154] trigger: parse.files.loop
  → [.\core1.symbolicBoot.lore:207] emit: parse.files.loop
  → [.\core1.symbolicBoot.lore:229] emit: parse.files.loop

🚨 Infinite loop detected on signal: 'signal.block'
  → [.\core.kernel.lore:18] trigger: signal.block
  → [.\core.kernel.lore:602] emit: signal.block
  → [.\core.kernel.lore:18] trigger: signal.block
  → [.\core.kernel.lore:602] emit: signal.block

🚨 Infinite loop detected on signal: 'echo.scan'
  → [.\core4.symbolicSubstrate.lore:16] emit: echo.scan
  → [.\core4.symbolicSubstrate.lore:255] trigger: echo.scan
  → [.\core4.symbolicSubstrate.lore:16] emit: echo.scan
  → [.\core4.symbolicSubstrate.lore:255] trigger: echo.scan

🚨 Infinite loop detected on signal: 'self.loop'
  → [.\core10.reflectiveSelf.lore:12] trigger: self.loop
  → [.\core10.reflectiveSelf.lore:40] emit: self.loop
  → [.\core10.reflectiveSelf.lore:56] when: self.loop
  → [.\core10.reflectiveSelf.lore:59] emit: self.loop
  → [.\core10.reflectiveSelf.lore:12] trigger: self.loop
  → [.\core10.reflectiveSelf.lore:40] emit: self.loop
  → [.\core10.reflectiveSelf.lore:56] when: self.loop
  → [.\core10.reflectiveSelf.lore:59] emit: self.loop

🚨 Infinite loop detected on signal: 'substrate.init'
  → [.\core4.symbolicSubstrate.lore:25] trigger: substrate.init
  → [.\core4.symbolicSubstrate.lore:488] emit: substrate.init
  → [.\core4.symbolicSubstrate.lore:25] trigger: substrate.init
  → [.\core4.symbolicSubstrate.lore:488] emit: substrate.init

🚨 Infinite loop detected on signal: 'snapshot.saved'
  → [.\core0.boot.lore:88] trigger: snapshot.saved
  → [.\core0.boot.lore:134] emit: snapshot.saved
  → [.\core0.boot.lore:88] trigger: snapshot.saved
  → [.\core0.boot.lore:134] emit: snapshot.saved

🚨 Infinite loop detected on signal: 'symbolic.react'
  → [.\core25.ioLayer.lore:106] emit: symbolic.react
  → [.\core25.ioLayer.lore:332] trigger: symbolic.react
  → [.\core25.ioLayer.lore:106] emit: symbolic.react
  → [.\core25.ioLayer.lore:332] trigger: symbolic.react

🚨 Infinite loop detected on signal: 'quantum.observe.memory'
  → [.\core5.echoMesh.lore:91] emit: quantum.observe.memory
  → [.\core9.quantumBranch.lore:18] trigger: quantum.observe.memory
  → [.\core5.echoMesh.lore:91] emit: quantum.observe.memory
  → [.\core9.quantumBranch.lore:18] trigger: quantum.observe.memory

🚨 Infinite loop detected on signal: 'detect.loot'
  → [.\core0.boot.lore:468] trigger: detect.loot
  → [.\core0.boot.lore:509] when: detect.loot
  → [.\core0.boot.lore:758] emit: detect.loot
  → [.\core0.boot.lore:468] trigger: detect.loot
  → [.\core0.boot.lore:509] when: detect.loot
  → [.\core0.boot.lore:758] emit: detect.loot

🚨 Infinite loop detected on signal: 'quantum.loop'
  → [.\core9.quantumBranch.lore:12] trigger: quantum.loop
  → [.\core9.quantumBranch.lore:44] emit: quantum.loop
  → [.\core9.quantumBranch.lore:63] emit: quantum.loop
  → [.\core9.quantumBranch.lore:12] trigger: quantum.loop
  → [.\core9.quantumBranch.lore:44] emit: quantum.loop
  → [.\core9.quantumBranch.lore:63] emit: quantum.loop

🚨 Infinite loop detected on signal: 'evolve.population'
  → [.\core.kernel.lore:23] trigger: evolve.population
  → [.\core.kernel.lore:240] emit: evolve.population
  → [.\core.kernel.lore:23] trigger: evolve.population
  → [.\core.kernel.lore:240] emit: evolve.population

🚨 Infinite loop detected on signal: 'location.history.view'
  → [.\core21.navigationLayer.lore:129] trigger: location.history.view
  → [.\core21.navigationLayer.lore:206] emit: location.history.view
  → [.\core21.navigationLayer.lore:129] trigger: location.history.view
  → [.\core21.navigationLayer.lore:206] emit: location.history.view

🚨 Infinite loop detected on signal: 'symbol.react'
  → [.\core24.symbolicCognition.lore:99] trigger: symbol.react
  → [.\core24.symbolicCognition.lore:411] emit: symbol.react
  → [.\core24.symbolicCognition.lore:99] trigger: symbol.react
  → [.\core24.symbolicCognition.lore:411] emit: symbol.react

🚨 Infinite loop detected on signal: 'input.resolve'
  → [.\core25.ioLayer.lore:75] emit: input.resolve
  → [.\core25.ioLayer.lore:83] emit: input.resolve
  → [.\core25.ioLayer.lore:88] trigger: input.resolve
  → [.\core25.ioLayer.lore:303] emit: input.resolve
  → [.\core25.ioLayer.lore:321] emit: input.resolve
  → [.\core25.ioLayer.lore:75] emit: input.resolve
  → [.\core25.ioLayer.lore:83] emit: input.resolve
  → [.\core25.ioLayer.lore:88] trigger: input.resolve
  → [.\core25.ioLayer.lore:303] emit: input.resolve
  → [.\core25.ioLayer.lore:321] emit: input.resolve

🚨 Infinite loop detected on signal: 'grammar.validate'
  → [.\core1.symbolicBoot.lore:25] emit: grammar.validate
  → [.\core12.languageLayer.lore:12] trigger: grammar.validate
  → [.\core12.languageLayer.lore:48] when: grammar.validate
  → [.\core1.symbolicBoot.lore:25] emit: grammar.validate
  → [.\core12.languageLayer.lore:12] trigger: grammar.validate
  → [.\core12.languageLayer.lore:48] when: grammar.validate

🚨 Infinite loop detected on signal: 'mirror.match.check'
  → [.\core14.contextReflector.lore:115] trigger: mirror.match.check
  → [.\core14.contextReflector.lore:486] emit: mirror.match.check
  → [.\core14.contextReflector.lore:115] trigger: mirror.match.check
  → [.\core14.contextReflector.lore:486] emit: mirror.match.check

🚨 Infinite loop detected on signal: 'narrative.context.append'
  → [.\core13.storyEngine.lore:343] trigger: narrative.context.append
  → [.\core13.storyEngine.lore:362] when: narrative.context.append
  → [.\core13.storyEngine.lore:589] emit: narrative.context.append
  → [.\core13.storyEngine.lore:343] trigger: narrative.context.append
  → [.\core13.storyEngine.lore:362] when: narrative.context.append
  → [.\core13.storyEngine.lore:589] emit: narrative.context.append

🚨 Infinite loop detected on signal: 'compile.extractPatterns'
  → [.\core22.symbolicCompiler.lore:21] emit: compile.extractPatterns
  → [.\core22.symbolicCompiler.lore:34] trigger: compile.extractPatterns
  → [.\core22.symbolicCompiler.lore:21] emit: compile.extractPatterns
  → [.\core22.symbolicCompiler.lore:34] trigger: compile.extractPatterns

🚨 Infinite loop detected on signal: 'load.boot'
  → [.\core0.boot.lore:470] trigger: load.boot
  → [.\core0.boot.lore:488] emit: load.boot
  → [.\core0.boot.lore:470] trigger: load.boot
  → [.\core0.boot.lore:488] emit: load.boot

🚨 Infinite loop detected on signal: 'mesh.init'
  → [.\core5.echoMesh.lore:7] emit: mesh.init
  → [.\core5.echoMesh.lore:13] trigger: mesh.init
  → [.\core5.echoMesh.lore:7] emit: mesh.init
  → [.\core5.echoMesh.lore:13] trigger: mesh.init

🚨 Infinite loop detected on signal: 'phaseOne'
  → [.\core0.boot.lore:273] emit: phaseOne
  → [.\core0.boot.lore:282] when: phaseOne
  → [.\core0.boot.lore:670] trigger: phaseOne
  → [.\core0.boot.lore:273] emit: phaseOne
  → [.\core0.boot.lore:282] when: phaseOne
  → [.\core0.boot.lore:670] trigger: phaseOne

🚨 Infinite loop detected on signal: 'reload.capsule'
  → [.\core7.reflectiveRuntime.lore:118] emit: reload.capsule
  → [.\core7.reflectiveRuntime.lore:131] trigger: reload.capsule
  → [.\core7.reflectiveRuntime.lore:118] emit: reload.capsule
  → [.\core7.reflectiveRuntime.lore:131] trigger: reload.capsule

🚨 Infinite loop detected on signal: 'context.pop.frame'
  → [.\core0.boot.lore:161] trigger: context.pop.frame
  → [.\core0.boot.lore:753] emit: context.pop.frame
  → [.\core0.boot.lore:161] trigger: context.pop.frame
  → [.\core0.boot.lore:753] emit: context.pop.frame

🚨 Infinite loop detected on signal: 'schedule.cancel'
  → [.\core15.executionFlow.lore:106] trigger: schedule.cancel
  → [.\core15.executionFlow.lore:402] emit: schedule.cancel
  → [.\core15.executionFlow.lore:106] trigger: schedule.cancel
  → [.\core15.executionFlow.lore:402] emit: schedule.cancel

🚨 Infinite loop detected on signal: 'emotion.cycle'
  → [.\core18.emotiveLayer.lore:181] trigger: emotion.cycle
  → [.\core18.emotiveLayer.lore:345] emit: emotion.cycle
  → [.\core18.emotiveLayer.lore:181] trigger: emotion.cycle
  → [.\core18.emotiveLayer.lore:345] emit: emotion.cycle

🚨 Infinite loop detected on signal: 'mood.changed'
  → [.\core18.emotiveLayer.lore:117] trigger: mood.changed
  → [.\core18.emotiveLayer.lore:350] emit: mood.changed
  → [.\core18.emotiveLayer.lore:117] trigger: mood.changed
  → [.\core18.emotiveLayer.lore:350] emit: mood.changed

🚨 Infinite loop detected on signal: 'glyph.listen'
  → [.\core.kernel.lore:56] trigger: glyph.listen
  → [.\core.kernel.lore:612] emit: glyph.listen
  → [.\core.kernel.lore:56] trigger: glyph.listen
  → [.\core.kernel.lore:612] emit: glyph.listen

🚨 Infinite loop detected on signal: 'cognition.dream'
  → [.\core6.echoCognition.lore:75] emit: cognition.dream
  → [.\core6.echoCognition.lore:190] trigger: cognition.dream
  → [.\core6.echoCognition.lore:75] emit: cognition.dream
  → [.\core6.echoCognition.lore:190] trigger: cognition.dream

🚨 Infinite loop detected on signal: 'evolve.cycle.trigger'
  → [.\core.kernel.lore:30] trigger: evolve.cycle.trigger
  → [.\core.kernel.lore:492] emit: evolve.cycle.trigger
  → [.\core.kernel.lore:30] trigger: evolve.cycle.trigger
  → [.\core.kernel.lore:492] emit: evolve.cycle.trigger

🚨 Infinite loop detected on signal: 'fertility.pollinate'
  → [.\core24.symbolicCognition.lore:309] trigger: fertility.pollinate
  → [.\core24.symbolicCognition.lore:386] emit: fertility.pollinate
  → [.\core24.symbolicCognition.lore:309] trigger: fertility.pollinate
  → [.\core24.symbolicCognition.lore:386] emit: fertility.pollinate

🚨 Infinite loop detected on signal: 'logic.pass'
  → [.\core17.choiceLogic.lore:79] emit: logic.pass
  → [.\core17.choiceLogic.lore:258] trigger: logic.pass
  → [.\core17.choiceLogic.lore:79] emit: logic.pass
  → [.\core17.choiceLogic.lore:258] trigger: logic.pass

🚨 Infinite loop detected on signal: 'sensor.stream'
  → [.\core25.ioLayer.lore:214] trigger: sensor.stream
  → [.\core25.ioLayer.lore:424] emit: sensor.stream
  → [.\core25.ioLayer.lore:214] trigger: sensor.stream
  → [.\core25.ioLayer.lore:424] emit: sensor.stream

🚨 Infinite loop detected on signal: 'loop.start'
  → [.\core15.executionFlow.lore:146] trigger: loop.start
  → [.\core15.executionFlow.lore:362] emit: loop.start
  → [.\core15.executionFlow.lore:146] trigger: loop.start
  → [.\core15.executionFlow.lore:362] emit: loop.start

🚨 Infinite loop detected on signal: 'audio.stop'
  → [.\core18.emotiveLayer.lore:75] trigger: audio.stop
  → [.\core18.emotiveLayer.lore:360] emit: audio.stop
  → [.\core18.emotiveLayer.lore:75] trigger: audio.stop
  → [.\core18.emotiveLayer.lore:360] emit: audio.stop

🚨 Infinite loop detected on signal: 'evaluate.triggers'
  → [.\core3.triggerEvaluator.lore:9] emit: evaluate.triggers
  → [.\core3.triggerEvaluator.lore:18] trigger: evaluate.triggers
  → [.\core3.triggerEvaluator.lore:9] emit: evaluate.triggers
  → [.\core3.triggerEvaluator.lore:18] trigger: evaluate.triggers

🚨 Infinite loop detected on signal: 'narrative.context.ready'
  → [.\core13.storyEngine.lore:360] emit: narrative.context.ready
  → [.\core13.storyEngine.lore:447] trigger: narrative.context.ready
  → [.\core13.storyEngine.lore:360] emit: narrative.context.ready
  → [.\core13.storyEngine.lore:447] trigger: narrative.context.ready

🚨 Infinite loop detected on signal: 'reason.inferCause'
  → [.\core16.signalInspector.lore:104] trigger: reason.inferCause
  → [.\core16.signalInspector.lore:376] emit: reason.inferCause
  → [.\core16.signalInspector.lore:104] trigger: reason.inferCause
  → [.\core16.signalInspector.lore:376] emit: reason.inferCause

🚨 Infinite loop detected on signal: 'narrative.snapshot'
  → [.\core13.storyEngine.lore:121] trigger: narrative.snapshot
  → [.\core13.storyEngine.lore:146] when: narrative.snapshot
  → [.\core13.storyEngine.lore:494] emit: narrative.snapshot
  → [.\core13.storyEngine.lore:121] trigger: narrative.snapshot
  → [.\core13.storyEngine.lore:146] when: narrative.snapshot
  → [.\core13.storyEngine.lore:494] emit: narrative.snapshot

🚨 Infinite loop detected on signal: 'ctx.onMatch'
  → [.\core14.contextReflector.lore:122] emit: ctx.onMatch
  → [.\core14.contextReflector.lore:369] trigger: ctx.onMatch
  → [.\core14.contextReflector.lore:122] emit: ctx.onMatch
  → [.\core14.contextReflector.lore:369] trigger: ctx.onMatch

🚨 Infinite loop detected on signal: 'scene.load.initial'
  → [.\core1.symbolicBoot.lore:74] emit: scene.load.initial
  → [.\core1.symbolicBoot.lore:342] trigger: scene.load.initial
  → [.\core1.symbolicBoot.lore:74] emit: scene.load.initial
  → [.\core1.symbolicBoot.lore:342] trigger: scene.load.initial

🚨 Infinite loop detected on signal: 'schedule.action'
  → [.\core15.executionFlow.lore:135] trigger: schedule.action
  → [.\core15.executionFlow.lore:382] emit: schedule.action
  → [.\core15.executionFlow.lore:135] trigger: schedule.action
  → [.\core15.executionFlow.lore:382] emit: schedule.action

🚨 Infinite loop detected on signal: 'stack.pop'
  → [.\core15.executionFlow.lore:216] trigger: stack.pop
  → [.\core15.executionFlow.lore:312] emit: stack.pop
  → [.\core15.executionFlow.lore:216] trigger: stack.pop
  → [.\core15.executionFlow.lore:312] emit: stack.pop

🚨 Infinite loop detected on signal: 'self.mirror.sync'
  → [.\core10.reflectiveSelf.lore:22] trigger: self.mirror.sync
  → [.\core10.reflectiveSelf.lore:118] when: self.mirror.sync
  → [.\core10.reflectiveSelf.lore:190] emit: self.mirror.sync
  → [.\core10.reflectiveSelf.lore:22] trigger: self.mirror.sync
  → [.\core10.reflectiveSelf.lore:118] when: self.mirror.sync
  → [.\core10.reflectiveSelf.lore:190] emit: self.mirror.sync

🚨 Infinite loop detected on signal: 'map.render'
  → [.\core21.navigationLayer.lore:30] trigger: map.render
  → [.\core21.navigationLayer.lore:221] emit: map.render
  → [.\core21.navigationLayer.lore:30] trigger: map.render
  → [.\core21.navigationLayer.lore:221] emit: map.render

🚨 Infinite loop detected on signal: 'input.reflect'
  → [.\core25.ioLayer.lore:318] trigger: input.reflect
  → [.\core25.ioLayer.lore:394] emit: input.reflect
  → [.\core25.ioLayer.lore:318] trigger: input.reflect
  → [.\core25.ioLayer.lore:394] emit: input.reflect

🚨 Infinite loop detected on signal: 'mesh.sync.quantum'
  → [.\core5.echoMesh.lore:182] emit: mesh.sync.quantum
  → [.\core5.echoMesh.lore:218] trigger: mesh.sync.quantum
  → [.\core5.echoMesh.lore:236] emit: mesh.sync.quantum
  → [.\core5.echoMesh.lore:182] emit: mesh.sync.quantum
  → [.\core5.echoMesh.lore:218] trigger: mesh.sync.quantum
  → [.\core5.echoMesh.lore:236] emit: mesh.sync.quantum

🚨 Infinite loop detected on signal: 'context.rewind'
  → [.\core14.contextReflector.lore:58] trigger: context.rewind
  → [.\core14.contextReflector.lore:441] emit: context.rewind
  → [.\core14.contextReflector.lore:58] trigger: context.rewind
  → [.\core14.contextReflector.lore:441] emit: context.rewind

🚨 Infinite loop detected on signal: 'memory.reflect.request'
  → [.\core5.echoMesh.lore:257] trigger: memory.reflect.request
  → [.\core5.echoMesh.lore:269] when: memory.reflect.request
  → [.\core5.echoMesh.lore:361] emit: memory.reflect.request
  → [.\core5.echoMesh.lore:257] trigger: memory.reflect.request
  → [.\core5.echoMesh.lore:269] when: memory.reflect.request
  → [.\core5.echoMesh.lore:361] emit: memory.reflect.request

🚨 Infinite loop detected on signal: 'learn.reset'
  → [.\core24.symbolicCognition.lore:73] trigger: learn.reset
  → [.\core24.symbolicCognition.lore:401] emit: learn.reset
  → [.\core24.symbolicCognition.lore:73] trigger: learn.reset
  → [.\core24.symbolicCognition.lore:401] emit: learn.reset

🚨 Infinite loop detected on signal: 'signal.route'
  → [.\core3.triggerEvaluator.lore:46] emit: signal.route
  → [.\core3.triggerEvaluator.lore:62] emit: signal.route
  → [.\core3.triggerEvaluator.lore:179] trigger: signal.route
  → [.\core3.triggerEvaluator.lore:46] emit: signal.route
  → [.\core3.triggerEvaluator.lore:62] emit: signal.route
  → [.\core3.triggerEvaluator.lore:179] trigger: signal.route

🚨 Infinite loop detected on signal: 'pulse.boot'
  → [.\glyphTutorial.lore:11] trigger: pulse.boot
  → [.\glyphTutorial.lore:232] emit: pulse.boot
  → [.\glyphTutorial.lore:11] trigger: pulse.boot
  → [.\glyphTutorial.lore:232] emit: pulse.boot

🚨 Infinite loop detected on signal: 'tutorial.launch.symbols'
  → [.\glyphTutorial.lore:10] trigger: tutorial.launch.symbols
  → [.\glyphTutorial.lore:237] emit: tutorial.launch.symbols
  → [.\glyphTutorial.lore:10] trigger: tutorial.launch.symbols
  → [.\glyphTutorial.lore:237] emit: tutorial.launch.symbols

🚨 Infinite loop detected on signal: 'sensor.read.auditory'
  → [.\core25.ioLayer.lore:258] trigger: sensor.read.auditory
  → [.\core25.ioLayer.lore:419] emit: sensor.read.auditory
  → [.\core25.ioLayer.lore:258] trigger: sensor.read.auditory
  → [.\core25.ioLayer.lore:419] emit: sensor.read.auditory

🚨 Infinite loop detected on signal: 'context.reflect'
  → [.\core14.contextReflector.lore:22] emit: context.reflect
  → [.\core14.contextReflector.lore:28] trigger: context.reflect
  → [.\core14.contextReflector.lore:62] emit: context.reflect
  → [.\core14.contextReflector.lore:69] emit: context.reflect
  → [.\core14.contextReflector.lore:22] emit: context.reflect
  → [.\core14.contextReflector.lore:28] trigger: context.reflect
  → [.\core14.contextReflector.lore:62] emit: context.reflect
  → [.\core14.contextReflector.lore:69] emit: context.reflect

🚨 Infinite loop detected on signal: 'story.arc.reveal'
  → [.\core13.storyEngine.lore:187] trigger: story.arc.reveal
  → [.\core13.storyEngine.lore:266] when: story.arc.reveal
  → [.\core13.storyEngine.lore:474] emit: story.arc.reveal
  → [.\core13.storyEngine.lore:187] trigger: story.arc.reveal
  → [.\core13.storyEngine.lore:266] when: story.arc.reveal
  → [.\core13.storyEngine.lore:474] emit: story.arc.reveal

🚨 Infinite loop detected on signal: 'input.replay'
  → [.\core25.ioLayer.lore:300] trigger: input.replay
  → [.\core25.ioLayer.lore:439] emit: input.replay
  → [.\core25.ioLayer.lore:300] trigger: input.replay
  → [.\core25.ioLayer.lore:439] emit: input.replay

🚨 Infinite loop detected on signal: 'mesh.sync.test'
  → [.\core5.echoMesh.lore:234] trigger: mesh.sync.test
  → [.\core5.echoMesh.lore:351] emit: mesh.sync.test
  → [.\core5.echoMesh.lore:234] trigger: mesh.sync.test
  → [.\core5.echoMesh.lore:351] emit: mesh.sync.test

🚨 Infinite loop detected on signal: 'cognition.boot'
  → [.\core6.echoCognition.lore:6] trigger: cognition.boot
  → [.\core6.echoCognition.lore:379] emit: cognition.boot
  → [.\core6.echoCognition.lore:6] trigger: cognition.boot
  → [.\core6.echoCognition.lore:379] emit: cognition.boot

🚨 Infinite loop detected on signal: 'quantum.ready'
  → [.\core9.quantumBranch.lore:46] emit: quantum.ready
  → [.\core9.quantumBranch.lore:56] emit: quantum.ready
  → [.\core9.quantumBranch.lore:205] trigger: quantum.ready
  → [.\core9.quantumBranch.lore:46] emit: quantum.ready
  → [.\core9.quantumBranch.lore:56] emit: quantum.ready
  → [.\core9.quantumBranch.lore:205] trigger: quantum.ready

🚨 Infinite loop detected on signal: 'debug.quantum.snapshot'
  → [.\core9.quantumBranch.lore:19] trigger: debug.quantum.snapshot
  → [.\core9.quantumBranch.lore:217] emit: debug.quantum.snapshot
  → [.\core9.quantumBranch.lore:19] trigger: debug.quantum.snapshot
  → [.\core9.quantumBranch.lore:217] emit: debug.quantum.snapshot

🚨 Infinite loop detected on signal: 'context.frame.saved'
  → [.\core0.boot.lore:164] trigger: context.frame.saved
  → [.\core0.boot.lore:187] emit: context.frame.saved
  → [.\core0.boot.lore:164] trigger: context.frame.saved
  → [.\core0.boot.lore:187] emit: context.frame.saved

🚨 Infinite loop detected on signal: 'version.patch'
  → [.\core22.symbolicCompiler.lore:25] emit: version.patch
  → [.\core22.symbolicCompiler.lore:313] trigger: version.patch
  → [.\core22.symbolicCompiler.lore:25] emit: version.patch
  → [.\core22.symbolicCompiler.lore:313] trigger: version.patch

🚨 Infinite loop detected on signal: 'resume.symbolic'
  → [.\core0.boot.lore:83] trigger: resume.symbolic
  → [.\core1.symbolicBoot.lore:72] emit: resume.symbolic
  → [.\core0.boot.lore:83] trigger: resume.symbolic
  → [.\core1.symbolicBoot.lore:72] emit: resume.symbolic

🚨 Infinite loop detected on signal: 'weave.checkpoint'
  → [.\core13.storyEngine.lore:396] trigger: weave.checkpoint
  → [.\core13.storyEngine.lore:412] emit: weave.checkpoint
  → [.\core13.storyEngine.lore:414] when: weave.checkpoint
  → [.\core13.storyEngine.lore:396] trigger: weave.checkpoint
  → [.\core13.storyEngine.lore:412] emit: weave.checkpoint
  → [.\core13.storyEngine.lore:414] when: weave.checkpoint

🚨 Infinite loop detected on signal: 'resonance.scan'
  → [.\core14.contextReflector.lore:202] trigger: resonance.scan
  → [.\core14.contextReflector.lore:406] emit: resonance.scan
  → [.\core14.contextReflector.lore:202] trigger: resonance.scan
  → [.\core14.contextReflector.lore:406] emit: resonance.scan

🚨 Infinite loop detected on signal: 'symbolic.execute'
  → [.\core11.capsuleExecutor.lore:120] trigger: symbolic.execute
  → [.\core11.capsuleExecutor.lore:131] trigger: symbolic.execute
  → [.\core2.capsuleEngine.lore:54] emit: symbolic.execute
  → [.\core11.capsuleExecutor.lore:120] trigger: symbolic.execute
  → [.\core11.capsuleExecutor.lore:131] trigger: symbolic.execute
  → [.\core2.capsuleEngine.lore:54] emit: symbolic.execute

🚨 Infinite loop detected on signal: 'symbolicSubstrate.route.logic'
  → [.\core4.symbolicSubstrate.lore:210] trigger: symbolicSubstrate.route.logic
  → [.\core4.symbolicSubstrate.lore:473] emit: symbolicSubstrate.route.logic
  → [.\core4.symbolicSubstrate.lore:210] trigger: symbolicSubstrate.route.logic
  → [.\core4.symbolicSubstrate.lore:473] emit: symbolicSubstrate.route.logic

🚨 Infinite loop detected on signal: 'loader.bootstrap'
  → [.\core0.boot.lore:41] trigger: loader.bootstrap
  → [.\core0.boot.lore:54] when: loader.bootstrap
  → [.\core0.boot.lore:69] emit: loader.bootstrap
  → [.\core0.boot.lore:41] trigger: loader.bootstrap
  → [.\core0.boot.lore:54] when: loader.bootstrap
  → [.\core0.boot.lore:69] emit: loader.bootstrap

🚨 Infinite loop detected on signal: 'self.reflect.memory'
  → [.\core10.reflectiveSelf.lore:15] trigger: self.reflect.memory
  → [.\core10.reflectiveSelf.lore:69] emit: self.reflect.memory
  → [.\core10.reflectiveSelf.lore:78] when: self.reflect.memory
  → [.\core10.reflectiveSelf.lore:15] trigger: self.reflect.memory
  → [.\core10.reflectiveSelf.lore:69] emit: self.reflect.memory
  → [.\core10.reflectiveSelf.lore:78] when: self.reflect.memory

🚨 Infinite loop detected on signal: 'timeline.define'
  → [.\core19.visualFlow.lore:49] trigger: timeline.define
  → [.\core19.visualFlow.lore:494] emit: timeline.define
  → [.\core19.visualFlow.lore:49] trigger: timeline.define
  → [.\core19.visualFlow.lore:494] emit: timeline.define

🚨 Infinite loop detected on signal: 'echo.resonate'
  → [.\core23.bridgeLayer.lore:390] emit: echo.resonate
  → [.\core23.bridgeLayer.lore:395] trigger: echo.resonate
  → [.\core23.bridgeLayer.lore:390] emit: echo.resonate
  → [.\core23.bridgeLayer.lore:395] trigger: echo.resonate

🚨 Infinite loop detected on signal: 'interaction.matrix.start'
  → [.\core20.symbolicEntity.lore:103] trigger: interaction.matrix.start
  → [.\core20.symbolicEntity.lore:260] emit: interaction.matrix.start
  → [.\core20.symbolicEntity.lore:103] trigger: interaction.matrix.start
  → [.\core20.symbolicEntity.lore:260] emit: interaction.matrix.start

🚨 Infinite loop detected on signal: 'input.intent'
  → [.\core6.echoCognition.lore:239] trigger: input.intent
  → [.\core6.echoCognition.lore:384] emit: input.intent
  → [.\core6.echoCognition.lore:239] trigger: input.intent
  → [.\core6.echoCognition.lore:384] emit: input.intent

🚨 Infinite loop detected on signal: 'capsule.quarantine'
  → [.\core.kernel.lore:41] trigger: capsule.quarantine
  → [.\core.kernel.lore:552] emit: capsule.quarantine
  → [.\core.kernel.lore:41] trigger: capsule.quarantine
  → [.\core.kernel.lore:552] emit: capsule.quarantine

🚨 Infinite loop detected on signal: 'tutorial.namedClosures'
  → [.\glyphTutorial.lore:145] emit: tutorial.namedClosures
  → [.\glyphTutorial.lore:195] trigger: tutorial.namedClosures
  → [.\glyphTutorial.lore:145] emit: tutorial.namedClosures
  → [.\glyphTutorial.lore:195] trigger: tutorial.namedClosures

🚨 Infinite loop detected on signal: 'signal.render'
  → [.\core.kernel.lore:16] trigger: signal.render
  → [.\core.kernel.lore:582] emit: signal.render
  → [.\core.kernel.lore:16] trigger: signal.render
  → [.\core.kernel.lore:582] emit: signal.render

🚨 Infinite loop detected on signal: 'kernel.execution.loop'
  → [.\core.kernel.lore:14] trigger: kernel.execution.loop
  → [.\core.kernel.lore:128] emit: kernel.execution.loop
  → [.\core.kernel.lore:14] trigger: kernel.execution.loop
  → [.\core.kernel.lore:128] emit: kernel.execution.loop

🚨 Infinite loop detected on signal: 'load.capsules'
  → [.\core1.symbolicBoot.lore:92] emit: load.capsules
  → [.\core1.symbolicBoot.lore:99] trigger: load.capsules
  → [.\core1.symbolicBoot.lore:92] emit: load.capsules
  → [.\core1.symbolicBoot.lore:99] trigger: load.capsules

🚨 Infinite loop detected on signal: 'causality.link'
  → [.\core13.storyEngine.lore:277] trigger: causality.link
  → [.\core13.storyEngine.lore:301] when: causality.link
  → [.\core13.storyEngine.lore:504] emit: causality.link
  → [.\core13.storyEngine.lore:277] trigger: causality.link
  → [.\core13.storyEngine.lore:301] when: causality.link
  → [.\core13.storyEngine.lore:504] emit: causality.link

🚨 Infinite loop detected on signal: 'thread.join'
  → [.\core15.executionFlow.lore:55] trigger: thread.join
  → [.\core15.executionFlow.lore:332] emit: thread.join
  → [.\core15.executionFlow.lore:55] trigger: thread.join
  → [.\core15.executionFlow.lore:332] emit: thread.join

🚨 Infinite loop detected on signal: 'capsule.introspect'
  → [.\core7.reflectiveRuntime.lore:98] trigger: capsule.introspect
  → [.\core7.reflectiveRuntime.lore:156] emit: capsule.introspect
  → [.\core7.reflectiveRuntime.lore:98] trigger: capsule.introspect
  → [.\core7.reflectiveRuntime.lore:156] emit: capsule.introspect

🚨 Infinite loop detected on signal: 'kernel.next'
  → [.\core1.symbolicBoot.lore:522] emit: kernel.next
  → [.\core1.symbolicBoot.lore:528] emit: kernel.next
  → [.\core1.symbolicBoot.lore:532] emit: kernel.next
  → [.\core1.symbolicBoot.lore:1094] trigger: kernel.next
  → [.\core1.symbolicBoot.lore:522] emit: kernel.next
  → [.\core1.symbolicBoot.lore:528] emit: kernel.next
  → [.\core1.symbolicBoot.lore:532] emit: kernel.next
  → [.\core1.symbolicBoot.lore:1094] trigger: kernel.next

🚨 Infinite loop detected on signal: 'echo.load.substate'
  → [.\core4.symbolicSubstrate.lore:77] emit: echo.load.substate
  → [.\core4.symbolicSubstrate.lore:319] trigger: echo.load.substate
  → [.\core4.symbolicSubstrate.lore:77] emit: echo.load.substate
  → [.\core4.symbolicSubstrate.lore:319] trigger: echo.load.substate

🚨 Infinite loop detected on signal: 'choiceLogic.idle'
  → [.\core17.choiceLogic.lore:192] emit: choiceLogic.idle
  → [.\core17.choiceLogic.lore:197] trigger: choiceLogic.idle
  → [.\core17.choiceLogic.lore:192] emit: choiceLogic.idle
  → [.\core17.choiceLogic.lore:197] trigger: choiceLogic.idle

🚨 Infinite loop detected on signal: 'signal.scope.end'
  → [.\core16.signalInspector.lore:40] trigger: signal.scope.end
  → [.\core16.signalInspector.lore:421] emit: signal.scope.end
  → [.\core16.signalInspector.lore:40] trigger: signal.scope.end
  → [.\core16.signalInspector.lore:421] emit: signal.scope.end

🚨 Infinite loop detected on signal: 'holoframe.define'
  → [.\core19.visualFlow.lore:232] trigger: holoframe.define
  → [.\core19.visualFlow.lore:429] emit: holoframe.define
  → [.\core19.visualFlow.lore:232] trigger: holoframe.define
  → [.\core19.visualFlow.lore:429] emit: holoframe.define

🚨 Infinite loop detected on signal: 'entity.recall'
  → [.\core20.symbolicEntity.lore:50] trigger: entity.recall
  → [.\core20.symbolicEntity.lore:265] emit: entity.recall
  → [.\core20.symbolicEntity.lore:50] trigger: entity.recall
  → [.\core20.symbolicEntity.lore:265] emit: entity.recall

🚨 Infinite loop detected on signal: 'every.2m.peerSweep'
  → [.\core23.bridgeLayer.lore:64] trigger: every.2m.peerSweep
  → [.\core23.bridgeLayer.lore:505] emit: every.2m.peerSweep
  → [.\core23.bridgeLayer.lore:64] trigger: every.2m.peerSweep
  → [.\core23.bridgeLayer.lore:505] emit: every.2m.peerSweep

🚨 Infinite loop detected on signal: 'boot.echoMesh'
  → [.\core5.echoMesh.lore:4] trigger: boot.echoMesh
  → [.\core5.echoMesh.lore:356] emit: boot.echoMesh
  → [.\core5.echoMesh.lore:4] trigger: boot.echoMesh
  → [.\core5.echoMesh.lore:356] emit: boot.echoMesh

🚨 Infinite loop detected on signal: 'echo.capture'
  → [.\core6.echoCognition.lore:261] trigger: echo.capture
  → [.\core6.echoCognition.lore:294] when: echo.capture
  → [.\core6.echoCognition.lore:359] emit: echo.capture
  → [.\core6.echoCognition.lore:261] trigger: echo.capture
  → [.\core6.echoCognition.lore:294] when: echo.capture
  → [.\core6.echoCognition.lore:359] emit: echo.capture

🚨 Infinite loop detected on signal: 'mood.cascade.flow'
  → [.\core18.emotiveLayer.lore:279] emit: mood.cascade.flow
  → [.\core18.emotiveLayer.lore:286] trigger: mood.cascade.flow
  → [.\core18.emotiveLayer.lore:279] emit: mood.cascade.flow
  → [.\core18.emotiveLayer.lore:286] trigger: mood.cascade.flow

🚨 Infinite loop detected on signal: 'web.fetch.result'
  → [.\core23.bridgeLayer.lore:199] emit: web.fetch.result
  → [.\core23.bridgeLayer.lore:212] emit: web.fetch.result
  → [.\core23.bridgeLayer.lore:217] trigger: web.fetch.result
  → [.\core23.bridgeLayer.lore:199] emit: web.fetch.result
  → [.\core23.bridgeLayer.lore:212] emit: web.fetch.result
  → [.\core23.bridgeLayer.lore:217] trigger: web.fetch.result

🚨 Infinite loop detected on signal: 'boot.capsuleLoader'
  → [.\core0.boot.assembly.lore:10] emit: boot.capsuleLoader
  → [.\core0.boot.lore:11] trigger: boot.capsuleLoader
  → [.\core0.boot.lore:763] emit: boot.capsuleLoader
  → [.\core0.boot.assembly.lore:10] emit: boot.capsuleLoader
  → [.\core0.boot.lore:11] trigger: boot.capsuleLoader
  → [.\core0.boot.lore:763] emit: boot.capsuleLoader

🚨 Infinite loop detected on signal: 'web.dom.update'
  → [.\core23.bridgeLayer.lore:237] trigger: web.dom.update
  → [.\core23.bridgeLayer.lore:450] emit: web.dom.update
  → [.\core23.bridgeLayer.lore:237] trigger: web.dom.update
  → [.\core23.bridgeLayer.lore:450] emit: web.dom.update

🚨 Infinite loop detected on signal: 'stack.peek'
  → [.\core15.executionFlow.lore:223] trigger: stack.peek
  → [.\core15.executionFlow.lore:317] emit: stack.peek
  → [.\core15.executionFlow.lore:223] trigger: stack.peek
  → [.\core15.executionFlow.lore:317] emit: stack.peek

🚨 Infinite loop detected on signal: 'quantumBranch.init'
  → [.\core9.quantumBranch.lore:20] trigger: quantumBranch.init
  → [.\core9.quantumBranch.lore:45] emit: quantumBranch.init
  → [.\core9.quantumBranch.lore:20] trigger: quantumBranch.init
  → [.\core9.quantumBranch.lore:45] emit: quantumBranch.init

🚨 Infinite loop detected on signal: 'timeline.play'
  → [.\core19.visualFlow.lore:61] trigger: timeline.play
  → [.\core19.visualFlow.lore:444] emit: timeline.play
  → [.\core19.visualFlow.lore:61] trigger: timeline.play
  → [.\core19.visualFlow.lore:444] emit: timeline.play

🚨 Infinite loop detected on signal: 'subsymbolic.route'
  → [.\core21.navigationLayer.lore:172] trigger: subsymbolic.route
  → [.\core21.navigationLayer.lore:226] emit: subsymbolic.route
  → [.\core21.navigationLayer.lore:172] trigger: subsymbolic.route
  → [.\core21.navigationLayer.lore:226] emit: subsymbolic.route

🚨 Infinite loop detected on signal: 'validate.grammar'
  → [.\grammar.lore:6] emit: validate.grammar
  → [.\grammar.lore:10] trigger: validate.grammar
  → [.\grammar.lore:6] emit: validate.grammar
  → [.\grammar.lore:10] trigger: validate.grammar

🚨 Infinite loop detected on signal: 'context.visualize'
  → [.\core14.contextReflector.lore:43] emit: context.visualize
  → [.\core14.contextReflector.lore:48] trigger: context.visualize
  → [.\core14.contextReflector.lore:43] emit: context.visualize
  → [.\core14.contextReflector.lore:48] trigger: context.visualize

🚨 Infinite loop detected on signal: 'capsuleExecutor.ready'
  → [.\core11.capsuleExecutor.lore:10] emit: capsuleExecutor.ready
  → [.\core11.capsuleExecutor.lore:31] trigger: capsuleExecutor.ready
  → [.\core11.capsuleExecutor.lore:10] emit: capsuleExecutor.ready
  → [.\core11.capsuleExecutor.lore:31] trigger: capsuleExecutor.ready

🚨 Infinite loop detected on signal: 'identity.matrix.ready'
  → [.\core14.contextReflector.lore:349] emit: identity.matrix.ready
  → [.\core14.contextReflector.lore:354] trigger: identity.matrix.ready
  → [.\core14.contextReflector.lore:349] emit: identity.matrix.ready
  → [.\core14.contextReflector.lore:354] trigger: identity.matrix.ready

🚨 Infinite loop detected on signal: 'echo.read'
  → [.\core4.symbolicSubstrate.lore:15] emit: echo.read
  → [.\core4.symbolicSubstrate.lore:239] trigger: echo.read
  → [.\core4.symbolicSubstrate.lore:15] emit: echo.read
  → [.\core4.symbolicSubstrate.lore:239] trigger: echo.read

🚨 Infinite loop detected on signal: 'traceEcho'
  → [.\core0.boot.lore:265] emit: traceEcho
  → [.\core0.boot.lore:276] when: traceEcho
  → [.\core0.boot.lore:664] trigger: traceEcho
  → [.\core0.boot.lore:265] emit: traceEcho
  → [.\core0.boot.lore:276] when: traceEcho
  → [.\core0.boot.lore:664] trigger: traceEcho

🚨 Infinite loop detected on signal: 'reflex.trigger'
  → [.\core18.emotiveLayer.lore:38] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:48] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:57] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:66] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:204] trigger: reflex.trigger
  → [.\core18.emotiveLayer.lore:38] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:48] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:57] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:66] emit: reflex.trigger
  → [.\core18.emotiveLayer.lore:204] trigger: reflex.trigger

🚨 Infinite loop detected on signal: 'map.define'
  → [.\core21.navigationLayer.lore:19] trigger: map.define
  → [.\core21.navigationLayer.lore:201] emit: map.define
  → [.\core21.navigationLayer.lore:19] trigger: map.define
  → [.\core21.navigationLayer.lore:201] emit: map.define

🚨 Infinite loop detected on signal: 'narrative.resume'
  → [.\core13.storyEngine.lore:122] trigger: narrative.resume
  → [.\core13.storyEngine.lore:150] when: narrative.resume
  → [.\core13.storyEngine.lore:574] emit: narrative.resume
  → [.\core13.storyEngine.lore:122] trigger: narrative.resume
  → [.\core13.storyEngine.lore:150] when: narrative.resume
  → [.\core13.storyEngine.lore:574] emit: narrative.resume

🚨 Infinite loop detected on signal: 'emotion.shift'
  → [.\core18.emotiveLayer.lore:138] trigger: emotion.shift
  → [.\core18.emotiveLayer.lore:355] emit: emotion.shift
  → [.\core18.emotiveLayer.lore:138] trigger: emotion.shift
  → [.\core18.emotiveLayer.lore:355] emit: emotion.shift

🚨 Infinite loop detected on signal: 'grammar.expand.variants'
  → [.\core12.languageLayer.lore:98] emit: grammar.expand.variants
  → [.\core12.languageLayer.lore:182] trigger: grammar.expand.variants
  → [.\core12.languageLayer.lore:98] emit: grammar.expand.variants
  → [.\core12.languageLayer.lore:182] trigger: grammar.expand.variants

🚨 Infinite loop detected on signal: 'capsule.describe'
  → [.\core22.symbolicCompiler.lore:169] trigger: capsule.describe
  → [.\core22.symbolicCompiler.lore:430] emit: capsule.describe
  → [.\core22.symbolicCompiler.lore:169] trigger: capsule.describe
  → [.\core22.symbolicCompiler.lore:430] emit: capsule.describe

🚨 Infinite loop detected on signal: 'ancestralReveal'
  → [.\core0.boot.lore:279] emit: ancestralReveal
  → [.\core0.boot.lore:286] when: ancestralReveal
  → [.\core0.boot.lore:279] emit: ancestralReveal
  → [.\core0.boot.lore:286] when: ancestralReveal

🚨 Infinite loop detected on signal: 'mirror.reflect'
  → [.\core5.echoMesh.lore:133] emit: mirror.reflect
  → [.\core5.echoMesh.lore:301] trigger: mirror.reflect
  → [.\core5.echoMesh.lore:133] emit: mirror.reflect
  → [.\core5.echoMesh.lore:301] trigger: mirror.reflect

🚨 Infinite loop detected on signal: 'boot.assembly'
  → [.\core0.boot.assembly.lore:3] trigger: boot.assembly
  → [.\core0.boot.assembly.lore:17] when: boot.assembly
  → [.\core0.boot.assembly.lore:27] emit: boot.assembly
  → [.\core0.boot.assembly.lore:3] trigger: boot.assembly
  → [.\core0.boot.assembly.lore:17] when: boot.assembly
  → [.\core0.boot.assembly.lore:27] emit: boot.assembly

🚨 Infinite loop detected on signal: 'mesh.sync'
  → [.\core10.reflectiveSelf.lore:121] emit: mesh.sync
  → [.\core5.echoMesh.lore:105] emit: mesh.sync
  → [.\core5.echoMesh.lore:160] emit: mesh.sync
  → [.\core5.echoMesh.lore:164] emit: mesh.sync
  → [.\core5.echoMesh.lore:169] trigger: mesh.sync
  → [.\core10.reflectiveSelf.lore:121] emit: mesh.sync
  → [.\core5.echoMesh.lore:105] emit: mesh.sync
  → [.\core5.echoMesh.lore:160] emit: mesh.sync
  → [.\core5.echoMesh.lore:164] emit: mesh.sync
  → [.\core5.echoMesh.lore:169] trigger: mesh.sync

🚨 Infinite loop detected on signal: 'evolve.select.best'
  → [.\core.kernel.lore:27] trigger: evolve.select.best
  → [.\core.kernel.lore:242] emit: evolve.select.best
  → [.\core.kernel.lore:27] trigger: evolve.select.best
  → [.\core.kernel.lore:242] emit: evolve.select.best

🚨 Infinite loop detected on signal: 'ctx.entry'
  → [.\core15.executionFlow.lore:50] emit: ctx.entry
  → [.\core15.executionFlow.lore:284] trigger: ctx.entry
  → [.\core15.executionFlow.lore:50] emit: ctx.entry
  → [.\core15.executionFlow.lore:284] trigger: ctx.entry

🚨 Infinite loop detected on signal: 'reason.breadcrumbs'
  → [.\core16.signalInspector.lore:122] trigger: reason.breadcrumbs
  → [.\core16.signalInspector.lore:321] emit: reason.breadcrumbs
  → [.\core16.signalInspector.lore:122] trigger: reason.breadcrumbs
  → [.\core16.signalInspector.lore:321] emit: reason.breadcrumbs

🚨 Infinite loop detected on signal: 'semantic.resolve'
  → [.\core12.languageLayer.lore:16] trigger: semantic.resolve
  → [.\core12.languageLayer.lore:112] when: semantic.resolve
  → [.\core12.languageLayer.lore:317] emit: semantic.resolve
  → [.\core12.languageLayer.lore:16] trigger: semantic.resolve
  → [.\core12.languageLayer.lore:112] when: semantic.resolve
  → [.\core12.languageLayer.lore:317] emit: semantic.resolve

🚨 Infinite loop detected on signal: 'input.alias'
  → [.\core25.ioLayer.lore:32] trigger: input.alias
  → [.\core25.ioLayer.lore:344] emit: input.alias
  → [.\core25.ioLayer.lore:32] trigger: input.alias
  → [.\core25.ioLayer.lore:344] emit: input.alias

🚨 Infinite loop detected on signal: 'map.link'
  → [.\core21.navigationLayer.lore:55] trigger: map.link
  → [.\core21.navigationLayer.lore:196] emit: map.link
  → [.\core21.navigationLayer.lore:55] trigger: map.link
  → [.\core21.navigationLayer.lore:196] emit: map.link

🚨 Infinite loop detected on signal: 'critical.signal.alert'
  → [.\core16.signalInspector.lore:213] trigger: critical.signal.alert
  → [.\core16.signalInspector.lore:336] emit: critical.signal.alert
  → [.\core16.signalInspector.lore:213] trigger: critical.signal.alert
  → [.\core16.signalInspector.lore:336] emit: critical.signal.alert

🚨 Infinite loop detected on signal: 'perf.pause'
  → [.\core26.symbolicMeta.lore:108] trigger: perf.pause
  → [.\core26.symbolicMeta.lore:256] emit: perf.pause
  → [.\core26.symbolicMeta.lore:108] trigger: perf.pause
  → [.\core26.symbolicMeta.lore:256] emit: perf.pause

🚨 Infinite loop detected on signal: 'draw.overlay'
  → [.\core19.visualFlow.lore:243] trigger: draw.overlay
  → [.\core19.visualFlow.lore:519] emit: draw.overlay
  → [.\core19.visualFlow.lore:243] trigger: draw.overlay
  → [.\core19.visualFlow.lore:519] emit: draw.overlay

🚨 Infinite loop detected on signal: 'reason.why'
  → [.\core16.signalInspector.lore:112] trigger: reason.why
  → [.\core16.signalInspector.lore:361] emit: reason.why
  → [.\core16.signalInspector.lore:112] trigger: reason.why
  → [.\core16.signalInspector.lore:361] emit: reason.why

🚨 Infinite loop detected on signal: 'draw.transition'
  → [.\core19.visualFlow.lore:147] trigger: draw.transition
  → [.\core19.visualFlow.lore:409] emit: draw.transition
  → [.\core19.visualFlow.lore:147] trigger: draw.transition
  → [.\core19.visualFlow.lore:409] emit: draw.transition

🚨 Infinite loop detected on signal: 'network.syncCapsule'
  → [.\core23.bridgeLayer.lore:126] trigger: network.syncCapsule
  → [.\core23.bridgeLayer.lore:470] emit: network.syncCapsule
  → [.\core23.bridgeLayer.lore:126] trigger: network.syncCapsule
  → [.\core23.bridgeLayer.lore:470] emit: network.syncCapsule

🚨 Infinite loop detected on signal: 'self.reflect.timeline'
  → [.\core10.reflectiveSelf.lore:16] trigger: self.reflect.timeline
  → [.\core10.reflectiveSelf.lore:70] emit: self.reflect.timeline
  → [.\core10.reflectiveSelf.lore:84] when: self.reflect.timeline
  → [.\core10.reflectiveSelf.lore:16] trigger: self.reflect.timeline
  → [.\core10.reflectiveSelf.lore:70] emit: self.reflect.timeline
  → [.\core10.reflectiveSelf.lore:84] when: self.reflect.timeline

🚨 Infinite loop detected on signal: 'mirror.realityCheck'
  → [.\core17.choiceLogic.lore:161] emit: mirror.realityCheck
  → [.\core17.choiceLogic.lore:270] trigger: mirror.realityCheck
  → [.\core17.choiceLogic.lore:161] emit: mirror.realityCheck
  → [.\core17.choiceLogic.lore:270] trigger: mirror.realityCheck

🚨 Infinite loop detected on signal: 'reflex.check'
  → [.\core18.emotiveLayer.lore:222] trigger: reflex.check
  → [.\core18.emotiveLayer.lore:330] emit: reflex.check
  → [.\core18.emotiveLayer.lore:222] trigger: reflex.check
  → [.\core18.emotiveLayer.lore:330] emit: reflex.check

🚨 Infinite loop detected on signal: 'intent.evaluate'
  → [.\core12.languageLayer.lore:135] emit: intent.evaluate
  → [.\core12.languageLayer.lore:138] when: intent.evaluate
  → [.\core6.echoCognition.lore:62] emit: intent.evaluate
  → [.\core6.echoCognition.lore:80] emit: intent.evaluate
  → [.\core6.echoCognition.lore:165] trigger: intent.evaluate
  → [.\core6.echoCognition.lore:244] emit: intent.evaluate
  → [.\core12.languageLayer.lore:135] emit: intent.evaluate
  → [.\core12.languageLayer.lore:138] when: intent.evaluate
  → [.\core6.echoCognition.lore:62] emit: intent.evaluate
  → [.\core6.echoCognition.lore:80] emit: intent.evaluate
  → [.\core6.echoCognition.lore:165] trigger: intent.evaluate
  → [.\core6.echoCognition.lore:244] emit: intent.evaluate

🚨 Infinite loop detected on signal: 'reflex.bind'
  → [.\core18.emotiveLayer.lore:192] trigger: reflex.bind
  → [.\core18.emotiveLayer.lore:340] emit: reflex.bind
  → [.\core18.emotiveLayer.lore:192] trigger: reflex.bind
  → [.\core18.emotiveLayer.lore:340] emit: reflex.bind

🚨 Infinite loop detected on signal: 'causality.capture'
  → [.\core13.storyEngine.lore:276] trigger: causality.capture
  → [.\core13.storyEngine.lore:292] when: causality.capture
  → [.\core13.storyEngine.lore:524] emit: causality.capture
  → [.\core13.storyEngine.lore:276] trigger: causality.capture
  → [.\core13.storyEngine.lore:292] when: causality.capture
  → [.\core13.storyEngine.lore:524] emit: causality.capture

🚨 Infinite loop detected on signal: 'echo.reflect'
  → [.\core6.echoCognition.lore:91] trigger: echo.reflect
  → [.\core6.echoCognition.lore:354] emit: echo.reflect
  → [.\core6.echoCognition.lore:91] trigger: echo.reflect
  → [.\core6.echoCognition.lore:354] emit: echo.reflect

🚨 Infinite loop detected on signal: 'grammar.symbol.sanitize'
  → [.\core12.languageLayer.lore:15] trigger: grammar.symbol.sanitize
  → [.\core12.languageLayer.lore:50] emit: grammar.symbol.sanitize
  → [.\core12.languageLayer.lore:74] when: grammar.symbol.sanitize
  → [.\core12.languageLayer.lore:15] trigger: grammar.symbol.sanitize
  → [.\core12.languageLayer.lore:50] emit: grammar.symbol.sanitize
  → [.\core12.languageLayer.lore:74] when: grammar.symbol.sanitize

🚨 Infinite loop detected on signal: 'quantum.init'
  → [.\core9.quantumBranch.lore:11] trigger: quantum.init
  → [.\core9.quantumBranch.lore:43] emit: quantum.init
  → [.\core9.quantumBranch.lore:11] trigger: quantum.init
  → [.\core9.quantumBranch.lore:43] emit: quantum.init

🚨 Infinite loop detected on signal: 'quantum.collapse.resolve'
  → [.\core9.quantumBranch.lore:16] trigger: quantum.collapse.resolve
  → [.\core9.quantumBranch.lore:85] emit: quantum.collapse.resolve
  → [.\core9.quantumBranch.lore:16] trigger: quantum.collapse.resolve
  → [.\core9.quantumBranch.lore:85] emit: quantum.collapse.resolve

🚨 Infinite loop detected on signal: 'logic.fail'
  → [.\core17.choiceLogic.lore:83] emit: logic.fail
  → [.\core17.choiceLogic.lore:276] trigger: logic.fail
  → [.\core17.choiceLogic.lore:83] emit: logic.fail
  → [.\core17.choiceLogic.lore:276] trigger: logic.fail

🚨 Infinite loop detected on signal: 'web.form.submit'
  → [.\core23.bridgeLayer.lore:175] trigger: web.form.submit
  → [.\core23.bridgeLayer.lore:480] emit: web.form.submit
  → [.\core23.bridgeLayer.lore:175] trigger: web.form.submit
  → [.\core23.bridgeLayer.lore:480] emit: web.form.submit

🚨 Infinite loop detected on signal: 'next.step'
  → [.\core0.lore:7] emit: next.step
  → [.\core0.lore:10] trigger: next.step
  → [.\core0.lore:7] emit: next.step
  → [.\core0.lore:10] trigger: next.step

🚨 Infinite loop detected on signal: 'self.observe.context'
  → [.\core10.reflectiveSelf.lore:20] trigger: self.observe.context
  → [.\core10.reflectiveSelf.lore:67] emit: self.observe.context
  → [.\core10.reflectiveSelf.lore:106] when: self.observe.context
  → [.\core10.reflectiveSelf.lore:20] trigger: self.observe.context
  → [.\core10.reflectiveSelf.lore:67] emit: self.observe.context
  → [.\core10.reflectiveSelf.lore:106] when: self.observe.context

🚨 Infinite loop detected on signal: 'signal.trace'
  → [.\core16.signalInspector.lore:15] trigger: signal.trace
  → [.\core16.signalInspector.lore:341] emit: signal.trace
  → [.\core16.signalInspector.lore:15] trigger: signal.trace
  → [.\core16.signalInspector.lore:341] emit: signal.trace

🚨 Infinite loop detected on signal: 'story.resolveChoice'
  → [.\core13.storyEngine.lore:177] trigger: story.resolveChoice
  → [.\core13.storyEngine.lore:210] emit: story.resolveChoice
  → [.\core13.storyEngine.lore:212] when: story.resolveChoice
  → [.\core13.storyEngine.lore:177] trigger: story.resolveChoice
  → [.\core13.storyEngine.lore:210] emit: story.resolveChoice
  → [.\core13.storyEngine.lore:212] when: story.resolveChoice

🚨 Infinite loop detected on signal: 'emotion.set'
  → [.\core18.emotiveLayer.lore:127] trigger: emotion.set
  → [.\core18.emotiveLayer.lore:375] emit: emotion.set
  → [.\core18.emotiveLayer.lore:127] trigger: emotion.set
  → [.\core18.emotiveLayer.lore:375] emit: emotion.set

🚨 Infinite loop detected on signal: 'contextReflector.init'
  → [.\core14.contextReflector.lore:7] emit: contextReflector.init
  → [.\core14.contextReflector.lore:12] trigger: contextReflector.init
  → [.\core14.contextReflector.lore:7] emit: contextReflector.init
  → [.\core14.contextReflector.lore:12] trigger: contextReflector.init

🚨 Infinite loop detected on signal: 'symbolic.routes.register'
  → [.\core4.symbolicSubstrate.lore:37] emit: symbolic.routes.register
  → [.\core4.symbolicSubstrate.lore:50] trigger: symbolic.routes.register
  → [.\core4.symbolicSubstrate.lore:37] emit: symbolic.routes.register
  → [.\core4.symbolicSubstrate.lore:50] trigger: symbolic.routes.register

🚨 Infinite loop detected on signal: 'validate.capsules'
  → [.\core0.boot.lore:389] trigger: validate.capsules
  → [.\core0.boot.lore:403] when: validate.capsules
  → [.\core0.boot.lore:449] emit: validate.capsules
  → [.\core1.symbolicBoot.lore:167] trigger: validate.capsules
  → [.\core1.symbolicBoot.lore:321] when: validate.capsules
  → [.\core1.symbolicBoot.lore:328] emit: validate.capsules
  → [.\core0.boot.lore:389] trigger: validate.capsules
  → [.\core0.boot.lore:403] when: validate.capsules
  → [.\core0.boot.lore:449] emit: validate.capsules
  → [.\core1.symbolicBoot.lore:167] trigger: validate.capsules
  → [.\core1.symbolicBoot.lore:321] when: validate.capsules
  → [.\core1.symbolicBoot.lore:328] emit: validate.capsules

🚨 Infinite loop detected on signal: 'visualFlow.init'
  → [.\core19.visualFlow.lore:9] emit: visualFlow.init
  → [.\core19.visualFlow.lore:16] trigger: visualFlow.init
  → [.\core19.visualFlow.lore:9] emit: visualFlow.init
  → [.\core19.visualFlow.lore:16] trigger: visualFlow.init

🚨 Infinite loop detected on signal: 'story.ready'
  → [.\core13.storyEngine.lore:65] emit: story.ready
  → [.\core13.storyEngine.lore:441] trigger: story.ready
  → [.\core13.storyEngine.lore:65] emit: story.ready
  → [.\core13.storyEngine.lore:441] trigger: story.ready

🚨 Infinite loop detected on signal: 'self.reflect.merge'
  → [.\core10.reflectiveSelf.lore:23] trigger: self.reflect.merge
  → [.\core10.reflectiveSelf.lore:130] emit: self.reflect.merge
  → [.\core10.reflectiveSelf.lore:134] when: self.reflect.merge
  → [.\core10.reflectiveSelf.lore:23] trigger: self.reflect.merge
  → [.\core10.reflectiveSelf.lore:130] emit: self.reflect.merge
  → [.\core10.reflectiveSelf.lore:134] when: self.reflect.merge

🚨 Infinite loop detected on signal: 'emotion.resonate'
  → [.\core18.emotiveLayer.lore:163] trigger: emotion.resonate
  → [.\core18.emotiveLayer.lore:320] emit: emotion.resonate
  → [.\core18.emotiveLayer.lore:163] trigger: emotion.resonate
  → [.\core18.emotiveLayer.lore:320] emit: emotion.resonate

🚨 Infinite loop detected on signal: 'guard.enforce'
  → [.\core0.boot.lore:309] trigger: guard.enforce
  → [.\core0.boot.lore:323] when: guard.enforce
  → [.\core1.symbolicBoot.lore:222] emit: guard.enforce
  → [.\core0.boot.lore:309] trigger: guard.enforce
  → [.\core0.boot.lore:323] when: guard.enforce
  → [.\core1.symbolicBoot.lore:222] emit: guard.enforce

🚨 Infinite loop detected on signal: 'dream.begin'
  → [.\core24.symbolicCognition.lore:130] trigger: dream.begin
  → [.\core24.symbolicCognition.lore:361] emit: dream.begin
  → [.\core24.symbolicCognition.lore:130] trigger: dream.begin
  → [.\core24.symbolicCognition.lore:361] emit: dream.begin

🚨 Infinite loop detected on signal: 'handlerRegistry.bind'
  → [.\core4.symbolicSubstrate.lore:94] emit: handlerRegistry.bind
  → [.\core4.symbolicSubstrate.lore:415] trigger: handlerRegistry.bind
  → [.\core4.symbolicSubstrate.lore:94] emit: handlerRegistry.bind
  → [.\core4.symbolicSubstrate.lore:415] trigger: handlerRegistry.bind

🚨 Infinite loop detected on signal: 'thread.start'
  → [.\core15.executionFlow.lore:4] trigger: thread.start
  → [.\core15.executionFlow.lore:352] emit: thread.start
  → [.\core15.executionFlow.lore:4] trigger: thread.start
  → [.\core15.executionFlow.lore:352] emit: thread.start

🚨 Infinite loop detected on signal: 'predict.signal'
  → [.\core24.symbolicCognition.lore:42] trigger: predict.signal
  → [.\core24.symbolicCognition.lore:371] emit: predict.signal
  → [.\core24.symbolicCognition.lore:42] trigger: predict.signal
  → [.\core24.symbolicCognition.lore:371] emit: predict.signal

🚨 Infinite loop detected on signal: 'choice.input'
  → [.\core17.choiceLogic.lore:204] trigger: choice.input
  → [.\core17.choiceLogic.lore:312] emit: choice.input
  → [.\core17.choiceLogic.lore:204] trigger: choice.input
  → [.\core17.choiceLogic.lore:312] emit: choice.input

🚨 Infinite loop detected on signal: 'capsule.inject'
  → [.\core1.symbolicBoot.lore:1240] emit: capsule.inject
  → [.\core26.symbolicMeta.lore:36] trigger: capsule.inject
  → [.\core26.symbolicMeta.lore:201] emit: capsule.inject
  → [.\core1.symbolicBoot.lore:1240] emit: capsule.inject
  → [.\core26.symbolicMeta.lore:36] trigger: capsule.inject
  → [.\core26.symbolicMeta.lore:201] emit: capsule.inject

🚨 Infinite loop detected on signal: 'mirror.self'
  → [.\core14.contextReflector.lore:141] trigger: mirror.self
  → [.\core14.contextReflector.lore:426] emit: mirror.self
  → [.\core14.contextReflector.lore:141] trigger: mirror.self
  → [.\core14.contextReflector.lore:426] emit: mirror.self

🚨 Infinite loop detected on signal: 'thread.status'
  → [.\core15.executionFlow.lore:38] trigger: thread.status
  → [.\core15.executionFlow.lore:347] emit: thread.status
  → [.\core15.executionFlow.lore:38] trigger: thread.status
  → [.\core15.executionFlow.lore:347] emit: thread.status

🚨 Infinite loop detected on signal: 'mesh.ping'
  → [.\core5.echoMesh.lore:42] emit: mesh.ping
  → [.\core5.echoMesh.lore:52] trigger: mesh.ping
  → [.\core5.echoMesh.lore:42] emit: mesh.ping
  → [.\core5.echoMesh.lore:52] trigger: mesh.ping

🚨 Infinite loop detected on signal: 'context.push.frame'
  → [.\core0.boot.lore:160] trigger: context.push.frame
  → [.\core0.boot.lore:768] emit: context.push.frame
  → [.\core0.boot.lore:160] trigger: context.push.frame
  → [.\core0.boot.lore:768] emit: context.push.frame

🚨 Infinite loop detected on signal: 'echoCognition.tick'
  → [.\core6.echoCognition.lore:37] emit: echoCognition.tick
  → [.\core6.echoCognition.lore:70] trigger: echoCognition.tick
  → [.\core6.echoCognition.lore:84] emit: echoCognition.tick
  → [.\core6.echoCognition.lore:37] emit: echoCognition.tick
  → [.\core6.echoCognition.lore:70] trigger: echoCognition.tick
  → [.\core6.echoCognition.lore:84] emit: echoCognition.tick

🚨 Infinite loop detected on signal: 'signalInspector.modify.watchList'
  → [.\core16.signalInspector.lore:261] trigger: signalInspector.modify.watchList
  → [.\core16.signalInspector.lore:311] emit: signalInspector.modify.watchList
  → [.\core16.signalInspector.lore:261] trigger: signalInspector.modify.watchList
  → [.\core16.signalInspector.lore:311] emit: signalInspector.modify.watchList

🚨 Infinite loop detected on signal: 'loop.phase.register'
  → [.\core15.executionFlow.lore:192] trigger: loop.phase.register
  → [.\core15.executionFlow.lore:297] emit: loop.phase.register
  → [.\core15.executionFlow.lore:192] trigger: loop.phase.register
  → [.\core15.executionFlow.lore:297] emit: loop.phase.register

🚨 Infinite loop detected on signal: 'mesh.sync.full'
  → [.\core5.echoMesh.lore:187] emit: mesh.sync.full
  → [.\core5.echoMesh.lore:193] trigger: mesh.sync.full
  → [.\core5.echoMesh.lore:238] emit: mesh.sync.full
  → [.\core5.echoMesh.lore:187] emit: mesh.sync.full
  → [.\core5.echoMesh.lore:193] trigger: mesh.sync.full
  → [.\core5.echoMesh.lore:238] emit: mesh.sync.full

🚨 Infinite loop detected on signal: 'schedule.at'
  → [.\core15.executionFlow.lore:71] trigger: schedule.at
  → [.\core15.executionFlow.lore:387] emit: schedule.at
  → [.\core15.executionFlow.lore:71] trigger: schedule.at
  → [.\core15.executionFlow.lore:387] emit: schedule.at

🚨 Infinite loop detected on signal: 'symbolic.routes.scan'
  → [.\core4.symbolicSubstrate.lore:36] emit: symbolic.routes.scan
  → [.\core4.symbolicSubstrate.lore:42] trigger: symbolic.routes.scan
  → [.\core4.symbolicSubstrate.lore:36] emit: symbolic.routes.scan
  → [.\core4.symbolicSubstrate.lore:42] trigger: symbolic.routes.scan

🚨 Infinite loop detected on signal: 'fs.extractCapsules'
  → [.\core1.symbolicBoot.lore:166] trigger: fs.extractCapsules
  → [.\core1.symbolicBoot.lore:188] emit: fs.extractCapsules
  → [.\core1.symbolicBoot.lore:166] trigger: fs.extractCapsules
  → [.\core1.symbolicBoot.lore:188] emit: fs.extractCapsules

🚨 Infinite loop detected on signal: 'entity.memory.get'
  → [.\core14.contextReflector.lore:257] trigger: entity.memory.get
  → [.\core14.contextReflector.lore:376] emit: entity.memory.get
  → [.\core14.contextReflector.lore:257] trigger: entity.memory.get
  → [.\core14.contextReflector.lore:376] emit: entity.memory.get

🚨 Infinite loop detected on signal: 'signalInspector.listen'
  → [.\core16.signalInspector.lore:237] emit: signalInspector.listen
  → [.\core16.signalInspector.lore:304] trigger: signalInspector.listen
  → [.\core16.signalInspector.lore:237] emit: signalInspector.listen
  → [.\core16.signalInspector.lore:304] trigger: signalInspector.listen

🚨 Infinite loop detected on signal: 'audio.react'
  → [.\core18.emotiveLayer.lore:27] trigger: audio.react
  → [.\core18.emotiveLayer.lore:120] emit: audio.react
  → [.\core18.emotiveLayer.lore:27] trigger: audio.react
  → [.\core18.emotiveLayer.lore:120] emit: audio.react

🚨 Infinite loop detected on signal: 'entity.memory.set'
  → [.\core14.contextReflector.lore:249] trigger: entity.memory.set
  → [.\core14.contextReflector.lore:421] emit: entity.memory.set
  → [.\core14.contextReflector.lore:249] trigger: entity.memory.set
  → [.\core14.contextReflector.lore:421] emit: entity.memory.set

🚨 Infinite loop detected on signal: 'signal.pause'
  → [.\core16.signalInspector.lore:87] trigger: signal.pause
  → [.\core16.signalInspector.lore:396] emit: signal.pause
  → [.\core16.signalInspector.lore:87] trigger: signal.pause
  → [.\core16.signalInspector.lore:396] emit: signal.pause

🚨 Infinite loop detected on signal: 'dream.sequence'
  → [.\core24.symbolicCognition.lore:148] emit: dream.sequence
  → [.\core24.symbolicCognition.lore:158] trigger: dream.sequence
  → [.\core24.symbolicCognition.lore:148] emit: dream.sequence
  → [.\core24.symbolicCognition.lore:158] trigger: dream.sequence

🚨 Infinite loop detected on signal: 'gravity.pull'
  → [.\core14.contextReflector.lore:170] emit: gravity.pull
  → [.\core14.contextReflector.lore:175] trigger: gravity.pull
  → [.\core14.contextReflector.lore:170] emit: gravity.pull
  → [.\core14.contextReflector.lore:175] trigger: gravity.pull

🚨 Infinite loop detected on signal: 'input.bindContext'
  → [.\core25.ioLayer.lore:127] trigger: input.bindContext
  → [.\core25.ioLayer.lore:359] emit: input.bindContext
  → [.\core25.ioLayer.lore:127] trigger: input.bindContext
  → [.\core25.ioLayer.lore:359] emit: input.bindContext

🚨 Infinite loop detected on signal: 'mirror.project'
  → [.\core17.choiceLogic.lore:143] emit: mirror.project
  → [.\core17.choiceLogic.lore:288] trigger: mirror.project
  → [.\core17.choiceLogic.lore:143] emit: mirror.project
  → [.\core17.choiceLogic.lore:288] trigger: mirror.project

🚨 Infinite loop detected on signal: 'perf.start'
  → [.\core26.symbolicMeta.lore:99] trigger: perf.start
  → [.\core26.symbolicMeta.lore:261] emit: perf.start
  → [.\core26.symbolicMeta.lore:99] trigger: perf.start
  → [.\core26.symbolicMeta.lore:261] emit: perf.start

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.grammar'
  → [.\core4.symbolicSubstrate.lore:11] emit: symbolicSubstrate.resolve.grammar
  → [.\core4.symbolicSubstrate.lore:153] trigger: symbolicSubstrate.resolve.grammar
  → [.\core4.symbolicSubstrate.lore:11] emit: symbolicSubstrate.resolve.grammar
  → [.\core4.symbolicSubstrate.lore:153] trigger: symbolicSubstrate.resolve.grammar

🚨 Infinite loop detected on signal: 'quantum.entanglement.evaluate'
  → [.\core9.quantumBranch.lore:17] trigger: quantum.entanglement.evaluate
  → [.\core9.quantumBranch.lore:69] emit: quantum.entanglement.evaluate
  → [.\core9.quantumBranch.lore:17] trigger: quantum.entanglement.evaluate
  → [.\core9.quantumBranch.lore:69] emit: quantum.entanglement.evaluate

🚨 Infinite loop detected on signal: 'parse.capsule.block'
  → [.\core1.symbolicBoot.lore:158] trigger: parse.capsule.block
  → [.\core1.symbolicBoot.lore:244] emit: parse.capsule.block
  → [.\core1.symbolicBoot.lore:158] trigger: parse.capsule.block
  → [.\core1.symbolicBoot.lore:244] emit: parse.capsule.block

🚨 Infinite loop detected on signal: 'reflex.triggered'
  → [.\core18.emotiveLayer.lore:229] emit: reflex.triggered
  → [.\core18.emotiveLayer.lore:242] trigger: reflex.triggered
  → [.\core18.emotiveLayer.lore:229] emit: reflex.triggered
  → [.\core18.emotiveLayer.lore:242] trigger: reflex.triggered

🚨 Infinite loop detected on signal: 'tutorial.glyph.reflective'
  → [.\glyphTutorial.lore:15] emit: tutorial.glyph.reflective
  → [.\glyphTutorial.lore:56] trigger: tutorial.glyph.reflective
  → [.\glyphTutorial.lore:15] emit: tutorial.glyph.reflective
  → [.\glyphTutorial.lore:56] trigger: tutorial.glyph.reflective

🚨 Infinite loop detected on signal: 'timeline.reset'
  → [.\core19.visualFlow.lore:102] trigger: timeline.reset
  → [.\core19.visualFlow.lore:394] emit: timeline.reset
  → [.\core19.visualFlow.lore:102] trigger: timeline.reset
  → [.\core19.visualFlow.lore:394] emit: timeline.reset

🚨 Infinite loop detected on signal: 'signal.intent.categorized'
  → [.\core12.languageLayer.lore:161] emit: signal.intent.categorized
  → [.\core12.languageLayer.lore:164] when: signal.intent.categorized
  → [.\core12.languageLayer.lore:295] trigger: signal.intent.categorized
  → [.\core12.languageLayer.lore:161] emit: signal.intent.categorized
  → [.\core12.languageLayer.lore:164] when: signal.intent.categorized
  → [.\core12.languageLayer.lore:295] trigger: signal.intent.categorized

🚨 Infinite loop detected on signal: 'system.boot'
  → [.\core.kernel.lore:10] trigger: system.boot
  → [.\core.kernel.lore:592] emit: system.boot
  → [.\core.kernel.lore:10] trigger: system.boot
  → [.\core.kernel.lore:592] emit: system.boot

🚨 Infinite loop detected on signal: 'story.flow.detect'
  → [.\core5.echoMesh.lore:116] emit: story.flow.detect
  → [.\core5.echoMesh.lore:307] trigger: story.flow.detect
  → [.\core5.echoMesh.lore:116] emit: story.flow.detect
  → [.\core5.echoMesh.lore:307] trigger: story.flow.detect

🚨 Infinite loop detected on signal: 'signal.resume'
  → [.\core16.signalInspector.lore:95] trigger: signal.resume
  → [.\core16.signalInspector.lore:426] emit: signal.resume
  → [.\core16.signalInspector.lore:95] trigger: signal.resume
  → [.\core16.signalInspector.lore:426] emit: signal.resume

🚨 Infinite loop detected on signal: 'engine.runNext'
  → [.\core2.capsuleEngine.lore:24] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:32] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:43] trigger: engine.runNext
  → [.\core2.capsuleEngine.lore:76] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:81] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:24] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:32] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:43] trigger: engine.runNext
  → [.\core2.capsuleEngine.lore:76] emit: engine.runNext
  → [.\core2.capsuleEngine.lore:81] emit: engine.runNext

🚨 Infinite loop detected on signal: 'context.overwrite'
  → [.\core26.symbolicMeta.lore:4] trigger: context.overwrite
  → [.\core26.symbolicMeta.lore:271] emit: context.overwrite
  → [.\core26.symbolicMeta.lore:4] trigger: context.overwrite
  → [.\core26.symbolicMeta.lore:271] emit: context.overwrite

🚨 Infinite loop detected on signal: 'schedule.check'
  → [.\core15.executionFlow.lore:122] trigger: schedule.check
  → [.\core15.executionFlow.lore:249] emit: schedule.check
  → [.\core15.executionFlow.lore:122] trigger: schedule.check
  → [.\core15.executionFlow.lore:249] emit: schedule.check

🚨 Infinite loop detected on signal: 'self.observe.identity'
  → [.\core10.reflectiveSelf.lore:19] trigger: self.observe.identity
  → [.\core10.reflectiveSelf.lore:66] emit: self.observe.identity
  → [.\core10.reflectiveSelf.lore:101] when: self.observe.identity
  → [.\core10.reflectiveSelf.lore:19] trigger: self.observe.identity
  → [.\core10.reflectiveSelf.lore:66] emit: self.observe.identity
  → [.\core10.reflectiveSelf.lore:101] when: self.observe.identity

🚨 Infinite loop detected on signal: 'sensor.read.visual'
  → [.\core25.ioLayer.lore:242] trigger: sensor.read.visual
  → [.\core25.ioLayer.lore:369] emit: sensor.read.visual
  → [.\core25.ioLayer.lore:242] trigger: sensor.read.visual
  → [.\core25.ioLayer.lore:369] emit: sensor.read.visual

🚨 Infinite loop detected on signal: 'quantum.branch.emit'
  → [.\core9.quantumBranch.lore:25] trigger: quantum.branch.emit
  → [.\core9.quantumBranch.lore:212] emit: quantum.branch.emit
  → [.\core9.quantumBranch.lore:25] trigger: quantum.branch.emit
  → [.\core9.quantumBranch.lore:212] emit: quantum.branch.emit

🚨 Infinite loop detected on signal: 'quantum.branch.debug'
  → [.\core9.quantumBranch.lore:27] trigger: quantum.branch.debug
  → [.\core9.quantumBranch.lore:227] emit: quantum.branch.debug
  → [.\core9.quantumBranch.lore:27] trigger: quantum.branch.debug
  → [.\core9.quantumBranch.lore:227] emit: quantum.branch.debug

🚨 Infinite loop detected on signal: 'evolve.fitness.score'
  → [.\core.kernel.lore:26] trigger: evolve.fitness.score
  → [.\core.kernel.lore:261] emit: evolve.fitness.score
  → [.\core.kernel.lore:26] trigger: evolve.fitness.score
  → [.\core.kernel.lore:261] emit: evolve.fitness.score

🚨 Infinite loop detected on signal: 'env.snapshot.ready'
  → [.\core4.symbolicSubstrate.lore:285] emit: env.snapshot.ready
  → [.\core4.symbolicSubstrate.lore:403] trigger: env.snapshot.ready
  → [.\core4.symbolicSubstrate.lore:285] emit: env.snapshot.ready
  → [.\core4.symbolicSubstrate.lore:403] trigger: env.snapshot.ready

🚨 Infinite loop detected on signal: 'cognition.reflect'
  → [.\core.kernel.lore:44] trigger: cognition.reflect
  → [.\core.kernel.lore:153] emit: cognition.reflect
  → [.\core.kernel.lore:44] trigger: cognition.reflect
  → [.\core.kernel.lore:153] emit: cognition.reflect

🚨 Infinite loop detected on signal: 'signal.active'
  → [.\core.kernel.lore:141] emit: signal.active
  → [.\core.kernel.lore:462] trigger: signal.active
  → [.\core.kernel.lore:141] emit: signal.active
  → [.\core.kernel.lore:462] trigger: signal.active

🚨 Infinite loop detected on signal: 'signal.intent.detect'
  → [.\core12.languageLayer.lore:19] trigger: signal.intent.detect
  → [.\core12.languageLayer.lore:158] when: signal.intent.detect
  → [.\core12.languageLayer.lore:302] emit: signal.intent.detect
  → [.\core12.languageLayer.lore:19] trigger: signal.intent.detect
  → [.\core12.languageLayer.lore:158] when: signal.intent.detect
  → [.\core12.languageLayer.lore:302] emit: signal.intent.detect

🚨 Infinite loop detected on signal: 'mirror.reflect.context'
  → [.\core14.contextReflector.lore:99] trigger: mirror.reflect.context
  → [.\core14.contextReflector.lore:481] emit: mirror.reflect.context
  → [.\core14.contextReflector.lore:99] trigger: mirror.reflect.context
  → [.\core14.contextReflector.lore:481] emit: mirror.reflect.context

🚨 Infinite loop detected on signal: 'choiceLogic.learn'
  → [.\core17.choiceLogic.lore:220] emit: choiceLogic.learn
  → [.\core17.choiceLogic.lore:226] trigger: choiceLogic.learn
  → [.\core17.choiceLogic.lore:220] emit: choiceLogic.learn
  → [.\core17.choiceLogic.lore:226] trigger: choiceLogic.learn

🚨 Infinite loop detected on signal: 'reject.signal'
  → [.\grammar.lore:53] emit: reject.signal
  → [.\grammar.lore:62] emit: reject.signal
  → [.\grammar.lore:70] trigger: reject.signal
  → [.\grammar.lore:53] emit: reject.signal
  → [.\grammar.lore:62] emit: reject.signal
  → [.\grammar.lore:70] trigger: reject.signal

🚨 Infinite loop detected on signal: 'tutorial.launch.modularBlocks'
  → [.\glyphTutorial.lore:141] trigger: tutorial.launch.modularBlocks
  → [.\glyphTutorial.lore:242] emit: tutorial.launch.modularBlocks
  → [.\glyphTutorial.lore:141] trigger: tutorial.launch.modularBlocks
  → [.\glyphTutorial.lore:242] emit: tutorial.launch.modularBlocks

🚨 Infinite loop detected on signal: 'capsuleLoader.begin'
  → [.\core0.boot.lore:40] trigger: capsuleLoader.begin
  → [.\core0.boot.lore:49] when: capsuleLoader.begin
  → [.\core0.boot.lore:68] emit: capsuleLoader.begin
  → [.\core0.boot.lore:40] trigger: capsuleLoader.begin
  → [.\core0.boot.lore:49] when: capsuleLoader.begin
  → [.\core0.boot.lore:68] emit: capsuleLoader.begin

🚨 Infinite loop detected on signal: 'guards.load'
  → [.\core0.boot.lore:308] trigger: guards.load
  → [.\core0.boot.lore:317] when: guards.load
  → [.\core0.boot.lore:748] emit: guards.load
  → [.\core0.boot.lore:308] trigger: guards.load
  → [.\core0.boot.lore:317] when: guards.load
  → [.\core0.boot.lore:748] emit: guards.load

🚨 Infinite loop detected on signal: 'entity.route'
  → [.\core20.symbolicEntity.lore:82] trigger: entity.route
  → [.\core20.symbolicEntity.lore:305] emit: entity.route
  → [.\core20.symbolicEntity.lore:82] trigger: entity.route
  → [.\core20.symbolicEntity.lore:305] emit: entity.route

🚨 Infinite loop detected on signal: 'boot.assembly.recurse'
  → [.\core0.boot.assembly.lore:4] trigger: boot.assembly.recurse
  → [.\core0.boot.assembly.lore:14] emit: boot.assembly.recurse
  → [.\core0.boot.assembly.lore:4] trigger: boot.assembly.recurse
  → [.\core0.boot.assembly.lore:14] emit: boot.assembly.recurse

🚨 Infinite loop detected on signal: 'compile.finalize'
  → [.\core22.symbolicCompiler.lore:23] emit: compile.finalize
  → [.\core22.symbolicCompiler.lore:56] trigger: compile.finalize
  → [.\core22.symbolicCompiler.lore:23] emit: compile.finalize
  → [.\core22.symbolicCompiler.lore:56] trigger: compile.finalize

🚨 Infinite loop detected on signal: 'grammar.coverage.scan'
  → [.\core12.languageLayer.lore:14] trigger: grammar.coverage.scan
  → [.\core12.languageLayer.lore:51] emit: grammar.coverage.scan
  → [.\core12.languageLayer.lore:84] when: grammar.coverage.scan
  → [.\core12.languageLayer.lore:96] emit: grammar.coverage.scan
  → [.\core12.languageLayer.lore:14] trigger: grammar.coverage.scan
  → [.\core12.languageLayer.lore:51] emit: grammar.coverage.scan
  → [.\core12.languageLayer.lore:84] when: grammar.coverage.scan
  → [.\core12.languageLayer.lore:96] emit: grammar.coverage.scan

🚨 Infinite loop detected on signal: 'grammar.evolve'
  → [.\core12.languageLayer.lore:13] trigger: grammar.evolve
  → [.\core12.languageLayer.lore:94] when: grammar.evolve
  → [.\core12.languageLayer.lore:312] emit: grammar.evolve
  → [.\core12.languageLayer.lore:13] trigger: grammar.evolve
  → [.\core12.languageLayer.lore:94] when: grammar.evolve
  → [.\core12.languageLayer.lore:312] emit: grammar.evolve

🚨 Infinite loop detected on signal: 'draw.3d.transform'
  → [.\core19.visualFlow.lore:352] trigger: draw.3d.transform
  → [.\core19.visualFlow.lore:469] emit: draw.3d.transform
  → [.\core19.visualFlow.lore:352] trigger: draw.3d.transform
  → [.\core19.visualFlow.lore:469] emit: draw.3d.transform

🚨 Infinite loop detected on signal: 'network.broadcast'
  → [.\core23.bridgeLayer.lore:118] trigger: network.broadcast
  → [.\core23.bridgeLayer.lore:465] emit: network.broadcast
  → [.\core23.bridgeLayer.lore:118] trigger: network.broadcast
  → [.\core23.bridgeLayer.lore:465] emit: network.broadcast

🚨 Infinite loop detected on signal: 'perf.diagnostics'
  → [.\core26.symbolicMeta.lore:71] emit: perf.diagnostics
  → [.\core26.symbolicMeta.lore:76] trigger: perf.diagnostics
  → [.\core26.symbolicMeta.lore:71] emit: perf.diagnostics
  → [.\core26.symbolicMeta.lore:76] trigger: perf.diagnostics

🚨 Infinite loop detected on signal: 'boot.webBridge'
  → [.\core0.boot.lore:62] emit: boot.webBridge
  → [.\core0.boot.lore:618] trigger: boot.webBridge
  → [.\core23.bridgeLayer.lore:303] trigger: boot.webBridge
  → [.\core0.boot.lore:62] emit: boot.webBridge
  → [.\core0.boot.lore:618] trigger: boot.webBridge
  → [.\core23.bridgeLayer.lore:303] trigger: boot.webBridge

🚨 Infinite loop detected on signal: 'snapshot.restore'
  → [.\core0.boot.lore:85] trigger: snapshot.restore
  → [.\core0.boot.lore:123] trigger: snapshot.restore
  → [.\core0.boot.lore:124] when: snapshot.restore
  → [.\core0.boot.lore:773] emit: snapshot.restore
  → [.\core0.boot.lore:85] trigger: snapshot.restore
  → [.\core0.boot.lore:123] trigger: snapshot.restore
  → [.\core0.boot.lore:124] when: snapshot.restore
  → [.\core0.boot.lore:773] emit: snapshot.restore

🚨 Infinite loop detected on signal: 'go'
  → [.\core1.symbolicBoot.lore:14] trigger: go
  → [.\core1.symbolicBoot.lore:109] trigger: go
  → [.\core1.symbolicBoot.lore:437] emit: go
  → [.\core1.symbolicBoot.lore:14] trigger: go
  → [.\core1.symbolicBoot.lore:109] trigger: go
  → [.\core1.symbolicBoot.lore:437] emit: go

🚨 Infinite loop detected on signal: 'input.contextualize'
  → [.\core25.ioLayer.lore:103] emit: input.contextualize
  → [.\core25.ioLayer.lore:111] trigger: input.contextualize
  → [.\core25.ioLayer.lore:103] emit: input.contextualize
  → [.\core25.ioLayer.lore:111] trigger: input.contextualize

🚨 Infinite loop detected on signal: 'env.snapshot'
  → [.\core4.symbolicSubstrate.lore:18] emit: env.snapshot
  → [.\core4.symbolicSubstrate.lore:280] trigger: env.snapshot
  → [.\core4.symbolicSubstrate.lore:18] emit: env.snapshot
  → [.\core4.symbolicSubstrate.lore:280] trigger: env.snapshot

🚨 Infinite loop detected on signal: 'draw.3d.object'
  → [.\core19.visualFlow.lore:320] trigger: draw.3d.object
  → [.\core19.visualFlow.lore:399] emit: draw.3d.object
  → [.\core19.visualFlow.lore:320] trigger: draw.3d.object
  → [.\core19.visualFlow.lore:399] emit: draw.3d.object

🚨 Infinite loop detected on signal: 'tutorial.symbolicBlockReuse'
  → [.\glyphTutorial.lore:144] emit: tutorial.symbolicBlockReuse
  → [.\glyphTutorial.lore:213] trigger: tutorial.symbolicBlockReuse
  → [.\glyphTutorial.lore:144] emit: tutorial.symbolicBlockReuse
  → [.\glyphTutorial.lore:213] trigger: tutorial.symbolicBlockReuse

🚨 Infinite loop detected on signal: 'choiceLogic.init'
  → [.\core17.choiceLogic.lore:177] emit: choiceLogic.init
  → [.\core17.choiceLogic.lore:182] trigger: choiceLogic.init
  → [.\core17.choiceLogic.lore:177] emit: choiceLogic.init
  → [.\core17.choiceLogic.lore:182] trigger: choiceLogic.init

🚨 Infinite loop detected on signal: 'draw.animateSequence'
  → [.\core19.visualFlow.lore:135] trigger: draw.animateSequence
  → [.\core19.visualFlow.lore:504] emit: draw.animateSequence
  → [.\core19.visualFlow.lore:135] trigger: draw.animateSequence
  → [.\core19.visualFlow.lore:504] emit: draw.animateSequence

🚨 Infinite loop detected on signal: 'version.diff'
  → [.\core22.symbolicCompiler.lore:301] trigger: version.diff
  → [.\core22.symbolicCompiler.lore:445] emit: version.diff
  → [.\core22.symbolicCompiler.lore:301] trigger: version.diff
  → [.\core22.symbolicCompiler.lore:445] emit: version.diff

🚨 Infinite loop detected on signal: 'echo.scan.ready'
  → [.\core4.symbolicSubstrate.lore:262] emit: echo.scan.ready
  → [.\core4.symbolicSubstrate.lore:367] trigger: echo.scan.ready
  → [.\core4.symbolicSubstrate.lore:262] emit: echo.scan.ready
  → [.\core4.symbolicSubstrate.lore:367] trigger: echo.scan.ready

🚨 Infinite loop detected on signal: 'signal.log'
  → [.\core16.signalInspector.lore:70] trigger: signal.log
  → [.\core16.signalInspector.lore:326] emit: signal.log
  → [.\core16.signalInspector.lore:70] trigger: signal.log
  → [.\core16.signalInspector.lore:326] emit: signal.log

🚨 Infinite loop detected on signal: 'draw.animate'
  → [.\core19.visualFlow.lore:124] trigger: draw.animate
  → [.\core19.visualFlow.lore:454] emit: draw.animate
  → [.\core19.visualFlow.lore:124] trigger: draw.animate
  → [.\core19.visualFlow.lore:454] emit: draw.animate

🚨 Infinite loop detected on signal: 'mesh.loop'
  → [.\core5.echoMesh.lore:8] emit: mesh.loop
  → [.\core5.echoMesh.lore:27] trigger: mesh.loop
  → [.\core5.echoMesh.lore:32] emit: mesh.loop
  → [.\core5.echoMesh.lore:45] emit: mesh.loop
  → [.\core5.echoMesh.lore:8] emit: mesh.loop
  → [.\core5.echoMesh.lore:27] trigger: mesh.loop
  → [.\core5.echoMesh.lore:32] emit: mesh.loop
  → [.\core5.echoMesh.lore:45] emit: mesh.loop

🚨 Infinite loop detected on signal: 'signal.scope.debug'
  → [.\core16.signalInspector.lore:50] trigger: signal.scope.debug
  → [.\core16.signalInspector.lore:431] emit: signal.scope.debug
  → [.\core16.signalInspector.lore:50] trigger: signal.scope.debug
  → [.\core16.signalInspector.lore:431] emit: signal.scope.debug

🚨 Infinite loop detected on signal: 'dream.loop.init'
  → [.\core.kernel.lore:50] trigger: dream.loop.init
  → [.\core.kernel.lore:127] emit: dream.loop.init
  → [.\core.kernel.lore:50] trigger: dream.loop.init
  → [.\core.kernel.lore:127] emit: dream.loop.init

🚨 Infinite loop detected on signal: 'net.react'
  → [.\core23.bridgeLayer.lore:35] emit: net.react
  → [.\core23.bridgeLayer.lore:41] trigger: net.react
  → [.\core23.bridgeLayer.lore:35] emit: net.react
  → [.\core23.bridgeLayer.lore:41] trigger: net.react

🚨 Infinite loop detected on signal: 'debug.memory.trace'
  → [.\core26.symbolicMeta.lore:179] trigger: debug.memory.trace
  → [.\core26.symbolicMeta.lore:236] emit: debug.memory.trace
  → [.\core26.symbolicMeta.lore:179] trigger: debug.memory.trace
  → [.\core26.symbolicMeta.lore:236] emit: debug.memory.trace

🚨 Infinite loop detected on signal: 'trace.init'
  → [.\core7.reflectiveRuntime.lore:7] emit: trace.init
  → [.\core7.reflectiveRuntime.lore:13] trigger: trace.init
  → [.\core7.reflectiveRuntime.lore:7] emit: trace.init
  → [.\core7.reflectiveRuntime.lore:13] trigger: trace.init

🚨 Infinite loop detected on signal: 'loot.exit'
  → [.\core0.boot.lore:514] emit: loot.exit
  → [.\core0.boot.lore:682] trigger: loot.exit
  → [.\core0.boot.lore:514] emit: loot.exit
  → [.\core0.boot.lore:682] trigger: loot.exit

🚨 Infinite loop detected on signal: 'trace.ready'
  → [.\core7.reflectiveRuntime.lore:27] emit: trace.ready
  → [.\core7.reflectiveRuntime.lore:34] trigger: trace.ready
  → [.\core7.reflectiveRuntime.lore:27] emit: trace.ready
  → [.\core7.reflectiveRuntime.lore:34] trigger: trace.ready

🚨 Infinite loop detected on signal: 'ctx.action'
  → [.\core20.symbolicEntity.lore:75] emit: ctx.action
  → [.\core20.symbolicEntity.lore:227] trigger: ctx.action
  → [.\core20.symbolicEntity.lore:75] emit: ctx.action
  → [.\core20.symbolicEntity.lore:227] trigger: ctx.action

🚨 Infinite loop detected on signal: 'patternRegistry.register'
  → [.\core4.symbolicSubstrate.lore:103] emit: patternRegistry.register
  → [.\core4.symbolicSubstrate.lore:421] trigger: patternRegistry.register
  → [.\core4.symbolicSubstrate.lore:103] emit: patternRegistry.register
  → [.\core4.symbolicSubstrate.lore:421] trigger: patternRegistry.register

🚨 Infinite loop detected on signal: 'fertility.germinate'
  → [.\core24.symbolicCognition.lore:243] emit: fertility.germinate
  → [.\core24.symbolicCognition.lore:251] trigger: fertility.germinate
  → [.\core24.symbolicCognition.lore:243] emit: fertility.germinate
  → [.\core24.symbolicCognition.lore:251] trigger: fertility.germinate

🚨 Infinite loop detected on signal: 'echoCognition.reflect.trace'
  → [.\core6.echoCognition.lore:74] emit: echoCognition.reflect.trace
  → [.\core6.echoCognition.lore:92] trigger: echoCognition.reflect.trace
  → [.\core6.echoCognition.lore:74] emit: echoCognition.reflect.trace
  → [.\core6.echoCognition.lore:92] trigger: echoCognition.reflect.trace

🚨 Infinite loop detected on signal: 'story.rewind'
  → [.\core13.storyEngine.lore:179] trigger: story.rewind
  → [.\core13.storyEngine.lore:220] when: story.rewind
  → [.\core13.storyEngine.lore:464] emit: story.rewind
  → [.\core13.storyEngine.lore:179] trigger: story.rewind
  → [.\core13.storyEngine.lore:220] when: story.rewind
  → [.\core13.storyEngine.lore:464] emit: story.rewind

🚨 Infinite loop detected on signal: 'every.30s.bridgePing'
  → [.\core23.bridgeLayer.lore:141] trigger: every.30s.bridgePing
  → [.\core23.bridgeLayer.lore:440] emit: every.30s.bridgePing
  → [.\core23.bridgeLayer.lore:141] trigger: every.30s.bridgePing
  → [.\core23.bridgeLayer.lore:440] emit: every.30s.bridgePing

🚨 Infinite loop detected on signal: 'entropy.check'
  → [.\core26.symbolicMeta.lore:116] trigger: entropy.check
  → [.\core26.symbolicMeta.lore:171] emit: entropy.check
  → [.\core26.symbolicMeta.lore:116] trigger: entropy.check
  → [.\core26.symbolicMeta.lore:171] emit: entropy.check

🚨 Infinite loop detected on signal: 'patternRegistry.scan'
  → [.\core4.symbolicSubstrate.lore:102] emit: patternRegistry.scan
  → [.\core4.symbolicSubstrate.lore:391] trigger: patternRegistry.scan
  → [.\core4.symbolicSubstrate.lore:102] emit: patternRegistry.scan
  → [.\core4.symbolicSubstrate.lore:391] trigger: patternRegistry.scan

🚨 Infinite loop detected on signal: 'device.storage.write'
  → [.\core23.bridgeLayer.lore:361] trigger: device.storage.write
  → [.\core23.bridgeLayer.lore:445] emit: device.storage.write
  → [.\core23.bridgeLayer.lore:361] trigger: device.storage.write
  → [.\core23.bridgeLayer.lore:445] emit: device.storage.write

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.echo'
  → [.\core4.symbolicSubstrate.lore:12] emit: symbolicSubstrate.resolve.echo
  → [.\core4.symbolicSubstrate.lore:172] trigger: symbolicSubstrate.resolve.echo
  → [.\core4.symbolicSubstrate.lore:12] emit: symbolicSubstrate.resolve.echo
  → [.\core4.symbolicSubstrate.lore:172] trigger: symbolicSubstrate.resolve.echo

🚨 Infinite loop detected on signal: 'echo.sync.mood'
  → [.\core5.echoMesh.lore:146] trigger: echo.sync.mood
  → [.\core5.echoMesh.lore:346] emit: echo.sync.mood
  → [.\core5.echoMesh.lore:146] trigger: echo.sync.mood
  → [.\core5.echoMesh.lore:346] emit: echo.sync.mood

🚨 Infinite loop detected on signal: 'cognition.init'
  → [.\core6.echoCognition.lore:10] emit: cognition.init
  → [.\core6.echoCognition.lore:19] trigger: cognition.init
  → [.\core6.echoCognition.lore:10] emit: cognition.init
  → [.\core6.echoCognition.lore:19] trigger: cognition.init

🚨 Infinite loop detected on signal: 'permissions.require'
  → [.\core.kernel.lore:32] trigger: permissions.require
  → [.\core.kernel.lore:572] emit: permissions.require
  → [.\core.kernel.lore:32] trigger: permissions.require
  → [.\core.kernel.lore:572] emit: permissions.require

🚨 Infinite loop detected on signal: 'semantic.bootstrap'
  → [.\core12.languageLayer.lore:35] emit: semantic.bootstrap
  → [.\core12.languageLayer.lore:107] when: semantic.bootstrap
  → [.\core12.languageLayer.lore:247] trigger: semantic.bootstrap
  → [.\core12.languageLayer.lore:35] emit: semantic.bootstrap
  → [.\core12.languageLayer.lore:107] when: semantic.bootstrap
  → [.\core12.languageLayer.lore:247] trigger: semantic.bootstrap

🚨 Infinite loop detected on signal: 'cap.token.validate'
  → [.\core.kernel.lore:38] trigger: cap.token.validate
  → [.\core.kernel.lore:487] emit: cap.token.validate
  → [.\core.kernel.lore:38] trigger: cap.token.validate
  → [.\core.kernel.lore:487] emit: cap.token.validate

🚨 Infinite loop detected on signal: 'context.analyze'
  → [.\core14.contextReflector.lore:34] emit: context.analyze
  → [.\core14.contextReflector.lore:39] trigger: context.analyze
  → [.\core14.contextReflector.lore:34] emit: context.analyze
  → [.\core14.contextReflector.lore:39] trigger: context.analyze

🚨 Infinite loop detected on signal: 'stack.clear'
  → [.\core15.executionFlow.lore:230] trigger: stack.clear
  → [.\core15.executionFlow.lore:397] emit: stack.clear
  → [.\core15.executionFlow.lore:230] trigger: stack.clear
  → [.\core15.executionFlow.lore:397] emit: stack.clear

🚨 Infinite loop detected on signal: 'parse.files.done'
  → [.\core1.symbolicBoot.lore:200] emit: parse.files.done
  → [.\core1.symbolicBoot.lore:366] trigger: parse.files.done
  → [.\core1.symbolicBoot.lore:200] emit: parse.files.done
  → [.\core1.symbolicBoot.lore:366] trigger: parse.files.done

🚨 Infinite loop detected on signal: 'mirror.overlay.scene'
  → [.\core14.contextReflector.lore:132] trigger: mirror.overlay.scene
  → [.\core14.contextReflector.lore:416] emit: mirror.overlay.scene
  → [.\core14.contextReflector.lore:132] trigger: mirror.overlay.scene
  → [.\core14.contextReflector.lore:416] emit: mirror.overlay.scene

🚨 Infinite loop detected on signal: 'version.snapshot'
  → [.\core22.symbolicCompiler.lore:334] trigger: version.snapshot
  → [.\core22.symbolicCompiler.lore:435] emit: version.snapshot
  → [.\core22.symbolicCompiler.lore:334] trigger: version.snapshot
  → [.\core22.symbolicCompiler.lore:435] emit: version.snapshot

🚨 Infinite loop detected on signal: 'io.sync'
  → [.\core25.ioLayer.lore:308] trigger: io.sync
  → [.\core25.ioLayer.lore:364] emit: io.sync
  → [.\core25.ioLayer.lore:308] trigger: io.sync
  → [.\core25.ioLayer.lore:364] emit: io.sync

🚨 Infinite loop detected on signal: 'input.inject'
  → [.\core25.ioLayer.lore:72] trigger: input.inject
  → [.\core25.ioLayer.lore:409] emit: input.inject
  → [.\core25.ioLayer.lore:72] trigger: input.inject
  → [.\core25.ioLayer.lore:409] emit: input.inject

🚨 Infinite loop detected on signal: 'memory.resolve.contexts'
  → [.\core4.symbolicSubstrate.lore:58] trigger: memory.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:503] emit: memory.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:58] trigger: memory.resolve.contexts
  → [.\core4.symbolicSubstrate.lore:503] emit: memory.resolve.contexts

🚨 Infinite loop detected on signal: 'evolve.evaluate.fitness'
  → [.\core.kernel.lore:25] trigger: evolve.evaluate.fitness
  → [.\core.kernel.lore:241] emit: evolve.evaluate.fitness
  → [.\core.kernel.lore:25] trigger: evolve.evaluate.fitness
  → [.\core.kernel.lore:241] emit: evolve.evaluate.fitness

🚨 Infinite loop detected on signal: 'fertility.seedBurst'
  → [.\core24.symbolicCognition.lore:282] trigger: fertility.seedBurst
  → [.\core24.symbolicCognition.lore:326] emit: fertility.seedBurst
  → [.\core24.symbolicCognition.lore:282] trigger: fertility.seedBurst
  → [.\core24.symbolicCognition.lore:326] emit: fertility.seedBurst

🚨 Infinite loop detected on signal: 'substrate.resolve'
  → [.\core.kernel.lore:59] trigger: substrate.resolve
  → [.\core.kernel.lore:542] emit: substrate.resolve
  → [.\core.kernel.lore:59] trigger: substrate.resolve
  → [.\core.kernel.lore:542] emit: substrate.resolve

🚨 Infinite loop detected on signal: 'validate.node'
  → [.\core11.capsuleExecutor.lore:59] emit: validate.node
  → [.\core11.capsuleExecutor.lore:143] trigger: validate.node
  → [.\core11.capsuleExecutor.lore:59] emit: validate.node
  → [.\core11.capsuleExecutor.lore:143] trigger: validate.node

🚨 Infinite loop detected on signal: 'narrative.init'
  → [.\core13.storyEngine.lore:12] emit: narrative.init
  → [.\core13.storyEngine.lore:118] trigger: narrative.init
  → [.\core13.storyEngine.lore:131] when: narrative.init
  → [.\core13.storyEngine.lore:12] emit: narrative.init
  → [.\core13.storyEngine.lore:118] trigger: narrative.init
  → [.\core13.storyEngine.lore:131] when: narrative.init

🚨 Infinite loop detected on signal: 'story.next'
  → [.\core13.storyEngine.lore:175] trigger: story.next
  → [.\core13.storyEngine.lore:201] emit: story.next
  → [.\core13.storyEngine.lore:203] when: story.next
  → [.\core13.storyEngine.lore:214] emit: story.next
  → [.\core13.storyEngine.lore:222] emit: story.next
  → [.\core13.storyEngine.lore:175] trigger: story.next
  → [.\core13.storyEngine.lore:201] emit: story.next
  → [.\core13.storyEngine.lore:203] when: story.next
  → [.\core13.storyEngine.lore:214] emit: story.next
  → [.\core13.storyEngine.lore:222] emit: story.next

🚨 Infinite loop detected on signal: 'story.logEvent'
  → [.\core13.storyEngine.lore:180] trigger: story.logEvent
  → [.\core13.storyEngine.lore:224] when: story.logEvent
  → [.\core13.storyEngine.lore:564] emit: story.logEvent
  → [.\core13.storyEngine.lore:180] trigger: story.logEvent
  → [.\core13.storyEngine.lore:224] when: story.logEvent
  → [.\core13.storyEngine.lore:564] emit: story.logEvent

🚨 Infinite loop detected on signal: 'time.tick'
  → [.\core15.executionFlow.lore:248] emit: time.tick
  → [.\core15.executionFlow.lore:272] trigger: time.tick
  → [.\core15.executionFlow.lore:248] emit: time.tick
  → [.\core15.executionFlow.lore:272] trigger: time.tick

🚨 Infinite loop detected on signal: 'cause.trigger'
  → [.\core13.storyEngine.lore:282] trigger: cause.trigger
  → [.\core13.storyEngine.lore:325] when: cause.trigger
  → [.\core13.storyEngine.lore:519] emit: cause.trigger
  → [.\core13.storyEngine.lore:282] trigger: cause.trigger
  → [.\core13.storyEngine.lore:325] when: cause.trigger
  → [.\core13.storyEngine.lore:519] emit: cause.trigger

🚨 Infinite loop detected on signal: 'thread.parallelSet'
  → [.\core15.executionFlow.lore:63] trigger: thread.parallelSet
  → [.\core15.executionFlow.lore:342] emit: thread.parallelSet
  → [.\core15.executionFlow.lore:63] trigger: thread.parallelSet
  → [.\core15.executionFlow.lore:342] emit: thread.parallelSet

🚨 Infinite loop detected on signal: 'ctx.outcome'
  → [.\core17.choiceLogic.lore:239] emit: ctx.outcome
  → [.\core17.choiceLogic.lore:300] trigger: ctx.outcome
  → [.\core17.choiceLogic.lore:239] emit: ctx.outcome
  → [.\core17.choiceLogic.lore:300] trigger: ctx.outcome

🚨 Infinite loop detected on signal: 'tick'
  → [.\core15.executionFlow.lore:89] trigger: tick
  → [.\core15.executionFlow.lore:372] emit: tick
  → [.\core15.executionFlow.lore:89] trigger: tick
  → [.\core15.executionFlow.lore:372] emit: tick

🚨 Infinite loop detected on signal: 'stack.push'
  → [.\core15.executionFlow.lore:209] trigger: stack.push
  → [.\core15.executionFlow.lore:302] emit: stack.push
  → [.\core15.executionFlow.lore:209] trigger: stack.push
  → [.\core15.executionFlow.lore:302] emit: stack.push

🚨 Infinite loop detected on signal: 'evolve.injectFitnessFn'
  → [.\core.kernel.lore:31] trigger: evolve.injectFitnessFn
  → [.\core.kernel.lore:502] emit: evolve.injectFitnessFn
  → [.\core.kernel.lore:31] trigger: evolve.injectFitnessFn
  → [.\core.kernel.lore:502] emit: evolve.injectFitnessFn

🚨 Infinite loop detected on signal: 'mind.plant'
  → [.\core20.symbolicEntity.lore:198] trigger: mind.plant
  → [.\core20.symbolicEntity.lore:290] emit: mind.plant
  → [.\core20.symbolicEntity.lore:198] trigger: mind.plant
  → [.\core20.symbolicEntity.lore:290] emit: mind.plant

🚨 Infinite loop detected on signal: 'scan.subsymbolic'
  → [.\core21.navigationLayer.lore:161] trigger: scan.subsymbolic
  → [.\core21.navigationLayer.lore:216] emit: scan.subsymbolic
  → [.\core21.navigationLayer.lore:161] trigger: scan.subsymbolic
  → [.\core21.navigationLayer.lore:216] emit: scan.subsymbolic

🚨 Infinite loop detected on signal: 'input.capture'
  → [.\core25.ioLayer.lore:64] trigger: input.capture
  → [.\core25.ioLayer.lore:389] emit: input.capture
  → [.\core25.ioLayer.lore:64] trigger: input.capture
  → [.\core25.ioLayer.lore:389] emit: input.capture

🚨 Infinite loop detected on signal: 'context.restored'
  → [.\core0.boot.lore:163] trigger: context.restored
  → [.\core0.boot.lore:175] emit: context.restored
  → [.\core0.boot.lore:163] trigger: context.restored
  → [.\core0.boot.lore:175] emit: context.restored

🚨 Infinite loop detected on signal: 'dream.tick'
  → [.\core24.symbolicCognition.lore:135] emit: dream.tick
  → [.\core24.symbolicCognition.lore:143] trigger: dream.tick
  → [.\core24.symbolicCognition.lore:150] emit: dream.tick
  → [.\core24.symbolicCognition.lore:135] emit: dream.tick
  → [.\core24.symbolicCognition.lore:143] trigger: dream.tick
  → [.\core24.symbolicCognition.lore:150] emit: dream.tick

🚨 Infinite loop detected on signal: 'story.observe'
  → [.\core13.storyEngine.lore:34] trigger: story.observe
  → [.\core13.storyEngine.lore:68] emit: story.observe
  → [.\core13.storyEngine.lore:72] when: story.observe
  → [.\core13.storyEngine.lore:34] trigger: story.observe
  → [.\core13.storyEngine.lore:68] emit: story.observe
  → [.\core13.storyEngine.lore:72] when: story.observe

🚨 Infinite loop detected on signal: 'ctx.nextSignal'
  → [.\core17.choiceLogic.lore:29] emit: ctx.nextSignal
  → [.\core17.choiceLogic.lore:264] trigger: ctx.nextSignal
  → [.\core17.choiceLogic.lore:29] emit: ctx.nextSignal
  → [.\core17.choiceLogic.lore:264] trigger: ctx.nextSignal

🚨 Infinite loop detected on signal: 'mind.resonate'
  → [.\core5.echoMesh.lore:106] emit: mind.resonate
  → [.\core5.echoMesh.lore:313] trigger: mind.resonate
  → [.\core5.echoMesh.lore:106] emit: mind.resonate
  → [.\core5.echoMesh.lore:313] trigger: mind.resonate

🚨 Infinite loop detected on signal: 'intent.observe'
  → [.\core12.languageLayer.lore:17] trigger: intent.observe
  → [.\core12.languageLayer.lore:132] when: intent.observe
  → [.\core12.languageLayer.lore:322] emit: intent.observe
  → [.\core12.languageLayer.lore:17] trigger: intent.observe
  → [.\core12.languageLayer.lore:132] when: intent.observe
  → [.\core12.languageLayer.lore:322] emit: intent.observe

🚨 Infinite loop detected on signal: 'evolve.apply'
  → [.\core.kernel.lore:28] trigger: evolve.apply
  → [.\core.kernel.lore:272] emit: evolve.apply
  → [.\core.kernel.lore:28] trigger: evolve.apply
  → [.\core.kernel.lore:272] emit: evolve.apply

🚨 Infinite loop detected on signal: 'tutorial.glyph.operators'
  → [.\glyphTutorial.lore:16] emit: tutorial.glyph.operators
  → [.\glyphTutorial.lore:75] trigger: tutorial.glyph.operators
  → [.\glyphTutorial.lore:16] emit: tutorial.glyph.operators
  → [.\glyphTutorial.lore:75] trigger: tutorial.glyph.operators

🚨 Infinite loop detected on signal: 'parser.scan'
  → [.\core1.symbolicBoot.lore:396] trigger: parser.scan
  → [.\core1.symbolicBoot.lore:560] emit: parser.scan
  → [.\core1.symbolicBoot.lore:396] trigger: parser.scan
  → [.\core1.symbolicBoot.lore:560] emit: parser.scan

🚨 Infinite loop detected on signal: 'draw.motion'
  → [.\core19.visualFlow.lore:158] trigger: draw.motion
  → [.\core19.visualFlow.lore:499] emit: draw.motion
  → [.\core19.visualFlow.lore:158] trigger: draw.motion
  → [.\core19.visualFlow.lore:499] emit: draw.motion

🚨 Infinite loop detected on signal: 'draw.fadeOut'
  → [.\core19.visualFlow.lore:199] trigger: draw.fadeOut
  → [.\core19.visualFlow.lore:419] emit: draw.fadeOut
  → [.\core19.visualFlow.lore:199] trigger: draw.fadeOut
  → [.\core19.visualFlow.lore:419] emit: draw.fadeOut

🚨 Infinite loop detected on signal: 'audio.speak'
  → [.\core18.emotiveLayer.lore:16] trigger: audio.speak
  → [.\core18.emotiveLayer.lore:335] emit: audio.speak
  → [.\core18.emotiveLayer.lore:16] trigger: audio.speak
  → [.\core18.emotiveLayer.lore:335] emit: audio.speak

🚨 Infinite loop detected on signal: 'mesh.observe'
  → [.\core5.echoMesh.lore:43] emit: mesh.observe
  → [.\core5.echoMesh.lore:100] trigger: mesh.observe
  → [.\core5.echoMesh.lore:43] emit: mesh.observe
  → [.\core5.echoMesh.lore:100] trigger: mesh.observe

🚨 Infinite loop detected on signal: 'memory.sync.full'
  → [.\core24.symbolicCognition.lore:273] emit: memory.sync.full
  → [.\core5.echoMesh.lore:154] trigger: memory.sync.full
  → [.\core24.symbolicCognition.lore:273] emit: memory.sync.full
  → [.\core5.echoMesh.lore:154] trigger: memory.sync.full

🚨 Infinite loop detected on signal: 'interpolate.done'
  → [.\core3.triggerEvaluator.lore:251] emit: interpolate.done
  → [.\core3.triggerEvaluator.lore:296] trigger: interpolate.done
  → [.\core3.triggerEvaluator.lore:251] emit: interpolate.done
  → [.\core3.triggerEvaluator.lore:296] trigger: interpolate.done

🚨 Infinite loop detected on signal: 'echo.write'
  → [.\core4.symbolicSubstrate.lore:17] emit: echo.write
  → [.\core4.symbolicSubstrate.lore:227] trigger: echo.write
  → [.\core4.symbolicSubstrate.lore:17] emit: echo.write
  → [.\core4.symbolicSubstrate.lore:227] trigger: echo.write

🚨 Infinite loop detected on signal: 'self.init'
  → [.\core10.reflectiveSelf.lore:11] trigger: self.init
  → [.\core10.reflectiveSelf.lore:39] emit: self.init
  → [.\core10.reflectiveSelf.lore:43] when: self.init
  → [.\core10.reflectiveSelf.lore:11] trigger: self.init
  → [.\core10.reflectiveSelf.lore:39] emit: self.init
  → [.\core10.reflectiveSelf.lore:43] when: self.init

🚨 Infinite loop detected on signal: 'weave.progress'
  → [.\core13.storyEngine.lore:395] trigger: weave.progress
  → [.\core13.storyEngine.lore:407] emit: weave.progress
  → [.\core13.storyEngine.lore:409] when: weave.progress
  → [.\core13.storyEngine.lore:395] trigger: weave.progress
  → [.\core13.storyEngine.lore:407] emit: weave.progress
  → [.\core13.storyEngine.lore:409] when: weave.progress

🚨 Infinite loop detected on signal: 'thread.run'
  → [.\core15.executionFlow.lore:14] trigger: thread.run
  → [.\core15.executionFlow.lore:377] emit: thread.run
  → [.\core15.executionFlow.lore:14] trigger: thread.run
  → [.\core15.executionFlow.lore:377] emit: thread.run

🚨 Infinite loop detected on signal: 'evolve.cycle.next'
  → [.\core.kernel.lore:29] trigger: evolve.cycle.next
  → [.\core.kernel.lore:278] emit: evolve.cycle.next
  → [.\core.kernel.lore:29] trigger: evolve.cycle.next
  → [.\core.kernel.lore:278] emit: evolve.cycle.next

🚨 Infinite loop detected on signal: 'signalInspector.view'
  → [.\core16.signalInspector.lore:282] trigger: signalInspector.view
  → [.\core16.signalInspector.lore:346] emit: signalInspector.view
  → [.\core16.signalInspector.lore:282] trigger: signalInspector.view
  → [.\core16.signalInspector.lore:346] emit: signalInspector.view

🚨 Infinite loop detected on signal: 'entity.remember'
  → [.\core20.symbolicEntity.lore:40] trigger: entity.remember
  → [.\core20.symbolicEntity.lore:270] emit: entity.remember
  → [.\core20.symbolicEntity.lore:40] trigger: entity.remember
  → [.\core20.symbolicEntity.lore:270] emit: entity.remember

🚨 Infinite loop detected on signal: 'dream.loop.tick'
  → [.\core.kernel.lore:45] trigger: dream.loop.tick
  → [.\core.kernel.lore:152] emit: dream.loop.tick
  → [.\core.kernel.lore:45] trigger: dream.loop.tick
  → [.\core.kernel.lore:152] emit: dream.loop.tick

🚨 Infinite loop detected on signal: 'net.pingAll'
  → [.\core23.bridgeLayer.lore:55] trigger: net.pingAll
  → [.\core23.bridgeLayer.lore:147] emit: net.pingAll
  → [.\core23.bridgeLayer.lore:55] trigger: net.pingAll
  → [.\core23.bridgeLayer.lore:147] emit: net.pingAll

🚨 Infinite loop detected on signal: 'mood.flow.capture'
  → [.\core13.storyEngine.lore:123] trigger: mood.flow.capture
  → [.\core13.storyEngine.lore:154] when: mood.flow.capture
  → [.\core13.storyEngine.lore:529] emit: mood.flow.capture
  → [.\core13.storyEngine.lore:123] trigger: mood.flow.capture
  → [.\core13.storyEngine.lore:154] when: mood.flow.capture
  → [.\core13.storyEngine.lore:529] emit: mood.flow.capture

🚨 Infinite loop detected on signal: 'echo.sync'
  → [.\core23.bridgeLayer.lore:386] trigger: echo.sync
  → [.\core25.ioLayer.lore:310] emit: echo.sync
  → [.\core23.bridgeLayer.lore:386] trigger: echo.sync
  → [.\core25.ioLayer.lore:310] emit: echo.sync

🚨 Infinite loop detected on signal: 'system.loadCapsules'
  → [.\core0.boot.lore:46] emit: system.loadCapsules
  → [.\core0.boot.lore:51] emit: system.loadCapsules
  → [.\core0.boot.lore:706] trigger: system.loadCapsules
  → [.\core1.symbolicBoot.lore:102] emit: system.loadCapsules
  → [.\core0.boot.lore:46] emit: system.loadCapsules
  → [.\core0.boot.lore:51] emit: system.loadCapsules
  → [.\core0.boot.lore:706] trigger: system.loadCapsules
  → [.\core1.symbolicBoot.lore:102] emit: system.loadCapsules

🚨 Infinite loop detected on signal: 'entropy.reset'
  → [.\core26.symbolicMeta.lore:124] trigger: entropy.reset
  → [.\core26.symbolicMeta.lore:251] emit: entropy.reset
  → [.\core26.symbolicMeta.lore:124] trigger: entropy.reset
  → [.\core26.symbolicMeta.lore:251] emit: entropy.reset

🚨 Infinite loop detected on signal: 'capsule.execute'
  → [.\core11.capsuleExecutor.lore:32] trigger: capsule.execute
  → [.\core3.triggerEvaluator.lore:143] emit: capsule.execute
  → [.\core11.capsuleExecutor.lore:32] trigger: capsule.execute
  → [.\core3.triggerEvaluator.lore:143] emit: capsule.execute

🚨 Infinite loop detected on signal: 'context.pop'
  → [.\core0.boot.lore:159] trigger: context.pop
  → [.\core0.boot.lore:743] emit: context.pop
  → [.\core0.boot.lore:159] trigger: context.pop
  → [.\core0.boot.lore:743] emit: context.pop

🚨 Infinite loop detected on signal: 'fs.parseCapsuleLines'
  → [.\core1.symbolicBoot.lore:165] trigger: fs.parseCapsuleLines
  → [.\core1.symbolicBoot.lore:182] emit: fs.parseCapsuleLines
  → [.\core1.symbolicBoot.lore:165] trigger: fs.parseCapsuleLines
  → [.\core1.symbolicBoot.lore:182] emit: fs.parseCapsuleLines

🚨 Infinite loop detected on signal: 'net.broadcast'
  → [.\core23.bridgeLayer.lore:18] emit: net.broadcast
  → [.\core23.bridgeLayer.lore:23] trigger: net.broadcast
  → [.\core23.bridgeLayer.lore:18] emit: net.broadcast
  → [.\core23.bridgeLayer.lore:23] trigger: net.broadcast

🚨 Infinite loop detected on signal: 'echo.cleared'
  → [.\core4.symbolicSubstrate.lore:274] emit: echo.cleared
  → [.\core4.symbolicSubstrate.lore:331] trigger: echo.cleared
  → [.\core4.symbolicSubstrate.lore:274] emit: echo.cleared
  → [.\core4.symbolicSubstrate.lore:331] trigger: echo.cleared

🚨 Infinite loop detected on signal: 'pulse.context'
  → [.\core14.contextReflector.lore:67] trigger: pulse.context
  → [.\core14.contextReflector.lore:391] emit: pulse.context
  → [.\core14.contextReflector.lore:67] trigger: pulse.context
  → [.\core14.contextReflector.lore:391] emit: pulse.context

🚨 Infinite loop detected on signal: 'mesh.resonance.check'
  → [.\core5.echoMesh.lore:243] trigger: mesh.resonance.check
  → [.\core5.echoMesh.lore:341] emit: mesh.resonance.check
  → [.\core5.echoMesh.lore:243] trigger: mesh.resonance.check
  → [.\core5.echoMesh.lore:341] emit: mesh.resonance.check

🚨 Infinite loop detected on signal: 'quantum.echo'
  → [.\core.kernel.lore:57] trigger: quantum.echo
  → [.\core.kernel.lore:497] emit: quantum.echo
  → [.\core.kernel.lore:57] trigger: quantum.echo
  → [.\core.kernel.lore:497] emit: quantum.echo

🚨 Infinite loop detected on signal: 'story.branch'
  → [.\core13.storyEngine.lore:184] trigger: story.branch
  → [.\core13.storyEngine.lore:245] when: story.branch
  → [.\core13.storyEngine.lore:584] emit: story.branch
  → [.\core13.storyEngine.lore:184] trigger: story.branch
  → [.\core13.storyEngine.lore:245] when: story.branch
  → [.\core13.storyEngine.lore:584] emit: story.branch

🚨 Infinite loop detected on signal: 'symbolic.checksum'
  → [.\core26.symbolicMeta.lore:173] emit: symbolic.checksum
  → [.\core26.symbolicMeta.lore:189] trigger: symbolic.checksum
  → [.\core26.symbolicMeta.lore:173] emit: symbolic.checksum
  → [.\core26.symbolicMeta.lore:189] trigger: symbolic.checksum

🚨 Infinite loop detected on signal: 'map.describe'
  → [.\core21.navigationLayer.lore:66] trigger: map.describe
  → [.\core21.navigationLayer.lore:186] emit: map.describe
  → [.\core21.navigationLayer.lore:66] trigger: map.describe
  → [.\core21.navigationLayer.lore:186] emit: map.describe

🚨 Infinite loop detected on signal: 'capsule.upgrade'
  → [.\core22.symbolicCompiler.lore:89] trigger: capsule.upgrade
  → [.\core22.symbolicCompiler.lore:415] emit: capsule.upgrade
  → [.\core22.symbolicCompiler.lore:89] trigger: capsule.upgrade
  → [.\core22.symbolicCompiler.lore:415] emit: capsule.upgrade

🚨 Infinite loop detected on signal: 'device.sensor.read'
  → [.\core23.bridgeLayer.lore:378] trigger: device.sensor.read
  → [.\core23.bridgeLayer.lore:430] emit: device.sensor.read
  → [.\core23.bridgeLayer.lore:378] trigger: device.sensor.read
  → [.\core23.bridgeLayer.lore:430] emit: device.sensor.read

🚨 Infinite loop detected on signal: 'capsule.reload'
  → [.\core7.reflectiveRuntime.lore:135] emit: capsule.reload
  → [.\core7.reflectiveRuntime.lore:149] trigger: capsule.reload
  → [.\core7.reflectiveRuntime.lore:135] emit: capsule.reload
  → [.\core7.reflectiveRuntime.lore:149] trigger: capsule.reload

🚨 Infinite loop detected on signal: 'agent.evaluate'
  → [.\core8.agenticLayer.lore:19] trigger: agent.evaluate
  → [.\core8.agenticLayer.lore:106] when: agent.evaluate
  → [.\core8.agenticLayer.lore:183] emit: agent.evaluate
  → [.\core8.agenticLayer.lore:19] trigger: agent.evaluate
  → [.\core8.agenticLayer.lore:106] when: agent.evaluate
  → [.\core8.agenticLayer.lore:183] emit: agent.evaluate

🚨 Infinite loop detected on signal: 'narrative.reflect'
  → [.\core13.storyEngine.lore:120] trigger: narrative.reflect
  → [.\core13.storyEngine.lore:141] when: narrative.reflect
  → [.\core13.storyEngine.lore:549] emit: narrative.reflect
  → [.\core13.storyEngine.lore:120] trigger: narrative.reflect
  → [.\core13.storyEngine.lore:141] when: narrative.reflect
  → [.\core13.storyEngine.lore:549] emit: narrative.reflect

🚨 Infinite loop detected on signal: 'cognition.ready'
  → [.\core6.echoCognition.lore:36] emit: cognition.ready
  → [.\core6.echoCognition.lore:45] trigger: cognition.ready
  → [.\core6.echoCognition.lore:36] emit: cognition.ready
  → [.\core6.echoCognition.lore:45] trigger: cognition.ready

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.logic'
  → [.\core.kernel.lore:216] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:9] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:28] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:121] trigger: symbolicSubstrate.resolve.logic
  → [.\core.kernel.lore:216] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:9] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:28] emit: symbolicSubstrate.resolve.logic
  → [.\core4.symbolicSubstrate.lore:121] trigger: symbolicSubstrate.resolve.logic

🚨 Infinite loop detected on signal: 'memory.snapshot.restore'
  → [.\core26.symbolicMeta.lore:156] trigger: memory.snapshot.restore
  → [.\core26.symbolicMeta.lore:266] emit: memory.snapshot.restore
  → [.\core26.symbolicMeta.lore:156] trigger: memory.snapshot.restore
  → [.\core26.symbolicMeta.lore:266] emit: memory.snapshot.restore

🚨 Infinite loop detected on signal: 'resonance.compare'
  → [.\core14.contextReflector.lore:205] emit: resonance.compare
  → [.\core14.contextReflector.lore:210] trigger: resonance.compare
  → [.\core14.contextReflector.lore:205] emit: resonance.compare
  → [.\core14.contextReflector.lore:210] trigger: resonance.compare

🚨 Infinite loop detected on signal: 'genome.express'
  → [.\core20.symbolicEntity.lore:161] trigger: genome.express
  → [.\core20.symbolicEntity.lore:275] emit: genome.express
  → [.\core20.symbolicEntity.lore:161] trigger: genome.express
  → [.\core20.symbolicEntity.lore:275] emit: genome.express

🚨 Infinite loop detected on signal: 'signal.reflect'
  → [.\core16.signalInspector.lore:58] trigger: signal.reflect
  → [.\core16.signalInspector.lore:411] emit: signal.reflect
  → [.\core16.signalInspector.lore:58] trigger: signal.reflect
  → [.\core16.signalInspector.lore:411] emit: signal.reflect

🚨 Infinite loop detected on signal: 'reason.explainCondition'
  → [.\core16.signalInspector.lore:130] trigger: reason.explainCondition
  → [.\core16.signalInspector.lore:371] emit: reason.explainCondition
  → [.\core16.signalInspector.lore:130] trigger: reason.explainCondition
  → [.\core16.signalInspector.lore:371] emit: reason.explainCondition

🚨 Infinite loop detected on signal: 'shell.statusReport'
  → [.\core19.visualFlow.lore:292] emit: shell.statusReport
  → [.\core19.visualFlow.lore:299] trigger: shell.statusReport
  → [.\core19.visualFlow.lore:292] emit: shell.statusReport
  → [.\core19.visualFlow.lore:299] trigger: shell.statusReport

🚨 Infinite loop detected on signal: 'render3d.update'
  → [.\core19.visualFlow.lore:313] emit: render3d.update
  → [.\core19.visualFlow.lore:324] emit: render3d.update
  → [.\core19.visualFlow.lore:335] emit: render3d.update
  → [.\core19.visualFlow.lore:362] trigger: render3d.update
  → [.\core19.visualFlow.lore:388] emit: render3d.update
  → [.\core19.visualFlow.lore:313] emit: render3d.update
  → [.\core19.visualFlow.lore:324] emit: render3d.update
  → [.\core19.visualFlow.lore:335] emit: render3d.update
  → [.\core19.visualFlow.lore:362] trigger: render3d.update
  → [.\core19.visualFlow.lore:388] emit: render3d.update

🚨 Infinite loop detected on signal: 'version.patch.applied'
  → [.\core22.symbolicCompiler.lore:317] emit: version.patch.applied
  → [.\core22.symbolicCompiler.lore:324] trigger: version.patch.applied
  → [.\core22.symbolicCompiler.lore:317] emit: version.patch.applied
  → [.\core22.symbolicCompiler.lore:324] trigger: version.patch.applied

🚨 Infinite loop detected on signal: 'web.route.byForm'
  → [.\core23.bridgeLayer.lore:178] emit: web.route.byForm
  → [.\core23.bridgeLayer.lore:183] trigger: web.route.byForm
  → [.\core23.bridgeLayer.lore:178] emit: web.route.byForm
  → [.\core23.bridgeLayer.lore:183] trigger: web.route.byForm

🚨 Infinite loop detected on signal: 'stack.snapshot'
  → [.\core24.symbolicCognition.lore:327] emit: stack.snapshot
  → [.\core24.symbolicCognition.lore:333] trigger: stack.snapshot
  → [.\core24.symbolicCognition.lore:327] emit: stack.snapshot
  → [.\core24.symbolicCognition.lore:333] trigger: stack.snapshot

🚨 Infinite loop detected on signal: 'mesh.resonance.wave'
  → [.\core5.echoMesh.lore:252] emit: mesh.resonance.wave
  → [.\core5.echoMesh.lore:319] trigger: mesh.resonance.wave
  → [.\core5.echoMesh.lore:252] emit: mesh.resonance.wave
  → [.\core5.echoMesh.lore:319] trigger: mesh.resonance.wave

🚨 Infinite loop detected on signal: 'sensor.readAll'
  → [.\core25.ioLayer.lore:209] emit: sensor.readAll
  → [.\core25.ioLayer.lore:233] trigger: sensor.readAll
  → [.\core25.ioLayer.lore:311] emit: sensor.readAll
  → [.\core25.ioLayer.lore:209] emit: sensor.readAll
  → [.\core25.ioLayer.lore:233] trigger: sensor.readAll
  → [.\core25.ioLayer.lore:311] emit: sensor.readAll

🚨 Infinite loop detected on signal: 'thread.list'
  → [.\core15.executionFlow.lore:22] trigger: thread.list
  → [.\core15.executionFlow.lore:337] emit: thread.list
  → [.\core15.executionFlow.lore:22] trigger: thread.list
  → [.\core15.executionFlow.lore:337] emit: thread.list

🚨 Infinite loop detected on signal: 'echoCognition.route'
  → [.\core6.echoCognition.lore:38] emit: echoCognition.route
  → [.\core6.echoCognition.lore:47] trigger: echoCognition.route
  → [.\core6.echoCognition.lore:38] emit: echoCognition.route
  → [.\core6.echoCognition.lore:47] trigger: echoCognition.route

🚨 Infinite loop detected on signal: 'mesh.tick'
  → [.\core5.echoMesh.lore:35] emit: mesh.tick
  → [.\core5.echoMesh.lore:40] trigger: mesh.tick
  → [.\core5.echoMesh.lore:35] emit: mesh.tick
  → [.\core5.echoMesh.lore:40] trigger: mesh.tick

🚨 Infinite loop detected on signal: 'symbolicSubstrate.bootstrap'
  → [.\core4.symbolicSubstrate.lore:4] trigger: symbolicSubstrate.bootstrap
  → [.\core4.symbolicSubstrate.lore:478] emit: symbolicSubstrate.bootstrap
  → [.\core4.symbolicSubstrate.lore:4] trigger: symbolicSubstrate.bootstrap
  → [.\core4.symbolicSubstrate.lore:478] emit: symbolicSubstrate.bootstrap

🚨 Infinite loop detected on signal: 'semantic.reflect'
  → [.\core12.languageLayer.lore:117] emit: semantic.reflect
  → [.\core12.languageLayer.lore:123] when: semantic.reflect
  → [.\core12.languageLayer.lore:289] trigger: semantic.reflect
  → [.\core12.languageLayer.lore:117] emit: semantic.reflect
  → [.\core12.languageLayer.lore:123] when: semantic.reflect
  → [.\core12.languageLayer.lore:289] trigger: semantic.reflect

🚨 Infinite loop detected on signal: 'narrative.context.init'
  → [.\core13.storyEngine.lore:342] trigger: narrative.context.init
  → [.\core13.storyEngine.lore:355] when: narrative.context.init
  → [.\core13.storyEngine.lore:459] emit: narrative.context.init
  → [.\core13.storyEngine.lore:342] trigger: narrative.context.init
  → [.\core13.storyEngine.lore:355] when: narrative.context.init
  → [.\core13.storyEngine.lore:459] emit: narrative.context.init

🚨 Infinite loop detected on signal: 'route.signal'
  → [.\core3.triggerEvaluator.lore:225] trigger: route.signal
  → [.\core3.triggerEvaluator.lore:309] emit: route.signal
  → [.\core3.triggerEvaluator.lore:225] trigger: route.signal
  → [.\core3.triggerEvaluator.lore:309] emit: route.signal

🚨 Infinite loop detected on signal: 'fertility.mutate'
  → [.\core24.symbolicCognition.lore:296] trigger: fertility.mutate
  → [.\core24.symbolicCognition.lore:366] emit: fertility.mutate
  → [.\core24.symbolicCognition.lore:296] trigger: fertility.mutate
  → [.\core24.symbolicCognition.lore:366] emit: fertility.mutate

🚨 Infinite loop detected on signal: 'memory.patch'
  → [.\core26.symbolicMeta.lore:28] trigger: memory.patch
  → [.\core26.symbolicMeta.lore:206] emit: memory.patch
  → [.\core26.symbolicMeta.lore:28] trigger: memory.patch
  → [.\core26.symbolicMeta.lore:206] emit: memory.patch

🚨 Infinite loop detected on signal: 'logic.fused'
  → [.\core17.choiceLogic.lore:63] emit: logic.fused
  → [.\core17.choiceLogic.lore:252] trigger: logic.fused
  → [.\core17.choiceLogic.lore:63] emit: logic.fused
  → [.\core17.choiceLogic.lore:252] trigger: logic.fused

🚨 Infinite loop detected on signal: 'version.restore.complete'
  → [.\core22.symbolicCompiler.lore:349] emit: version.restore.complete
  → [.\core22.symbolicCompiler.lore:356] trigger: version.restore.complete
  → [.\core22.symbolicCompiler.lore:349] emit: version.restore.complete
  → [.\core22.symbolicCompiler.lore:356] trigger: version.restore.complete

🚨 Infinite loop detected on signal: 'context.compare'
  → [.\core14.contextReflector.lore:82] trigger: context.compare
  → [.\core14.contextReflector.lore:386] emit: context.compare
  → [.\core14.contextReflector.lore:82] trigger: context.compare
  → [.\core14.contextReflector.lore:386] emit: context.compare

🚨 Infinite loop detected on signal: 'signal.scope.capture'
  → [.\core16.signalInspector.lore:32] trigger: signal.scope.capture
  → [.\core16.signalInspector.lore:316] emit: signal.scope.capture
  → [.\core16.signalInspector.lore:32] trigger: signal.scope.capture
  → [.\core16.signalInspector.lore:316] emit: signal.scope.capture

🚨 Infinite loop detected on signal: 'witness.observe'
  → [.\core16.signalInspector.lore:174] trigger: witness.observe
  → [.\core16.signalInspector.lore:351] emit: witness.observe
  → [.\core16.signalInspector.lore:174] trigger: witness.observe
  → [.\core16.signalInspector.lore:351] emit: witness.observe

🚨 Infinite loop detected on signal: 'intent.sync'
  → [.\core12.languageLayer.lore:36] emit: intent.sync
  → [.\core12.languageLayer.lore:211] trigger: intent.sync
  → [.\core12.languageLayer.lore:36] emit: intent.sync
  → [.\core12.languageLayer.lore:211] trigger: intent.sync

🚨 Infinite loop detected on signal: 'version.restore'
  → [.\core22.symbolicCompiler.lore:26] emit: version.restore
  → [.\core22.symbolicCompiler.lore:345] trigger: version.restore
  → [.\core22.symbolicCompiler.lore:26] emit: version.restore
  → [.\core22.symbolicCompiler.lore:345] trigger: version.restore

🚨 Infinite loop detected on signal: 'intent.define'
  → [.\core12.languageLayer.lore:18] trigger: intent.define
  → [.\core12.languageLayer.lore:150] when: intent.define
  → [.\core12.languageLayer.lore:307] emit: intent.define
  → [.\core12.languageLayer.lore:18] trigger: intent.define
  → [.\core12.languageLayer.lore:150] when: intent.define
  → [.\core12.languageLayer.lore:307] emit: intent.define

🚨 Infinite loop detected on signal: 'initiateMetamorphosis'
  → [.\core0.boot.lore:257] emit: initiateMetamorphosis
  → [.\core0.boot.lore:270] when: initiateMetamorphosis
  → [.\core0.boot.lore:658] trigger: initiateMetamorphosis
  → [.\core0.boot.lore:257] emit: initiateMetamorphosis
  → [.\core0.boot.lore:270] when: initiateMetamorphosis
  → [.\core0.boot.lore:658] trigger: initiateMetamorphosis

🚨 Infinite loop detected on signal: 'schedule.list'
  → [.\core15.executionFlow.lore:114] trigger: schedule.list
  → [.\core15.executionFlow.lore:367] emit: schedule.list
  → [.\core15.executionFlow.lore:114] trigger: schedule.list
  → [.\core15.executionFlow.lore:367] emit: schedule.list

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.triggers'
  → [.\core4.symbolicSubstrate.lore:10] emit: symbolicSubstrate.resolve.triggers
  → [.\core4.symbolicSubstrate.lore:134] trigger: symbolicSubstrate.resolve.triggers
  → [.\core4.symbolicSubstrate.lore:10] emit: symbolicSubstrate.resolve.triggers
  → [.\core4.symbolicSubstrate.lore:134] trigger: symbolicSubstrate.resolve.triggers

🚨 Infinite loop detected on signal: 'grammar.coverage.complete'
  → [.\core12.languageLayer.lore:91] emit: grammar.coverage.complete
  → [.\core12.languageLayer.lore:229] trigger: grammar.coverage.complete
  → [.\core12.languageLayer.lore:91] emit: grammar.coverage.complete
  → [.\core12.languageLayer.lore:229] trigger: grammar.coverage.complete

🚨 Infinite loop detected on signal: 'substrate.reset'
  → [.\core4.symbolicSubstrate.lore:19] emit: substrate.reset
  → [.\core4.symbolicSubstrate.lore:291] trigger: substrate.reset
  → [.\core4.symbolicSubstrate.lore:19] emit: substrate.reset
  → [.\core4.symbolicSubstrate.lore:291] trigger: substrate.reset

🚨 Infinite loop detected on signal: 'symbolicSubstrate.bind.routes'
  → [.\core4.symbolicSubstrate.lore:29] emit: symbolicSubstrate.bind.routes
  → [.\core4.symbolicSubstrate.lore:34] trigger: symbolicSubstrate.bind.routes
  → [.\core4.symbolicSubstrate.lore:29] emit: symbolicSubstrate.bind.routes
  → [.\core4.symbolicSubstrate.lore:34] trigger: symbolicSubstrate.bind.routes

🚨 Infinite loop detected on signal: 'render3d.refresh.viewport'
  → [.\core19.visualFlow.lore:365] emit: render3d.refresh.viewport
  → [.\core19.visualFlow.lore:373] trigger: render3d.refresh.viewport
  → [.\core19.visualFlow.lore:365] emit: render3d.refresh.viewport
  → [.\core19.visualFlow.lore:373] trigger: render3d.refresh.viewport

🚨 Infinite loop detected on signal: 'overlay.clear'
  → [.\core19.visualFlow.lore:254] trigger: overlay.clear
  → [.\core19.visualFlow.lore:514] emit: overlay.clear
  → [.\core19.visualFlow.lore:254] trigger: overlay.clear
  → [.\core19.visualFlow.lore:514] emit: overlay.clear

🚨 Infinite loop detected on signal: 'dna.transcribe'
  → [.\core20.symbolicEntity.lore:172] trigger: dna.transcribe
  → [.\core20.symbolicEntity.lore:295] emit: dna.transcribe
  → [.\core20.symbolicEntity.lore:172] trigger: dna.transcribe
  → [.\core20.symbolicEntity.lore:295] emit: dna.transcribe

🚨 Infinite loop detected on signal: 'require.entitlement'
  → [.\core.kernel.lore:36] trigger: require.entitlement
  → [.\core.kernel.lore:547] emit: require.entitlement
  → [.\core.kernel.lore:36] trigger: require.entitlement
  → [.\core.kernel.lore:547] emit: require.entitlement

🚨 Infinite loop detected on signal: 'compile.optimizeStructure'
  → [.\core22.symbolicCompiler.lore:22] emit: compile.optimizeStructure
  → [.\core22.symbolicCompiler.lore:45] trigger: compile.optimizeStructure
  → [.\core22.symbolicCompiler.lore:22] emit: compile.optimizeStructure
  → [.\core22.symbolicCompiler.lore:45] trigger: compile.optimizeStructure

🚨 Infinite loop detected on signal: 'web.fetch.post'
  → [.\core23.bridgeLayer.lore:204] trigger: web.fetch.post
  → [.\core23.bridgeLayer.lore:455] emit: web.fetch.post
  → [.\core23.bridgeLayer.lore:204] trigger: web.fetch.post
  → [.\core23.bridgeLayer.lore:455] emit: web.fetch.post

🚨 Infinite loop detected on signal: 'trace.playback'
  → [.\core24.symbolicCognition.lore:85] trigger: trace.playback
  → [.\core24.symbolicCognition.lore:391] emit: trace.playback
  → [.\core24.symbolicCognition.lore:85] trigger: trace.playback
  → [.\core24.symbolicCognition.lore:391] emit: trace.playback

🚨 Infinite loop detected on signal: 'input.queue'
  → [.\core25.ioLayer.lore:47] trigger: input.queue
  → [.\core25.ioLayer.lore:354] emit: input.queue
  → [.\core25.ioLayer.lore:47] trigger: input.queue
  → [.\core25.ioLayer.lore:354] emit: input.queue

🚨 Infinite loop detected on signal: 'ghost.listen'
  → [.\core.kernel.lore:42] trigger: ghost.listen
  → [.\core4.symbolicSubstrate.lore:69] emit: ghost.listen
  → [.\core.kernel.lore:42] trigger: ghost.listen
  → [.\core4.symbolicSubstrate.lore:69] emit: ghost.listen

🚨 Infinite loop detected on signal: 'net.receive'
  → [.\core23.bridgeLayer.lore:31] trigger: net.receive
  → [.\core23.bridgeLayer.lore:460] emit: net.receive
  → [.\core23.bridgeLayer.lore:31] trigger: net.receive
  → [.\core23.bridgeLayer.lore:460] emit: net.receive

🚨 Infinite loop detected on signal: 'trace.loop'
  → [.\core7.reflectiveRuntime.lore:8] emit: trace.loop
  → [.\core7.reflectiveRuntime.lore:35] trigger: trace.loop
  → [.\core7.reflectiveRuntime.lore:41] emit: trace.loop
  → [.\core7.reflectiveRuntime.lore:8] emit: trace.loop
  → [.\core7.reflectiveRuntime.lore:35] trigger: trace.loop
  → [.\core7.reflectiveRuntime.lore:41] emit: trace.loop

🚨 Infinite loop detected on signal: 'entity.exit'
  → [.\core20.symbolicEntity.lore:92] trigger: entity.exit
  → [.\core20.symbolicEntity.lore:250] emit: entity.exit
  → [.\core20.symbolicEntity.lore:92] trigger: entity.exit
  → [.\core20.symbolicEntity.lore:250] emit: entity.exit

🚨 Infinite loop detected on signal: 'web.dom.clear'
  → [.\core23.bridgeLayer.lore:247] trigger: web.dom.clear
  → [.\core23.bridgeLayer.lore:435] emit: web.dom.clear
  → [.\core23.bridgeLayer.lore:247] trigger: web.dom.clear
  → [.\core23.bridgeLayer.lore:435] emit: web.dom.clear

🚨 Infinite loop detected on signal: 'reflex.define'
  → [.\core18.emotiveLayer.lore:263] trigger: reflex.define
  → [.\core18.emotiveLayer.lore:365] emit: reflex.define
  → [.\core18.emotiveLayer.lore:263] trigger: reflex.define
  → [.\core18.emotiveLayer.lore:365] emit: reflex.define

🚨 Infinite loop detected on signal: 'capsule.parse'
  → [.\core1.symbolicBoot.lore:1179] emit: capsule.parse
  → [.\core1.symbolicBoot.lore:1287] trigger: capsule.parse
  → [.\core1.symbolicBoot.lore:1179] emit: capsule.parse
  → [.\core1.symbolicBoot.lore:1287] trigger: capsule.parse

🚨 Infinite loop detected on signal: 'input.flush'
  → [.\core25.ioLayer.lore:54] trigger: input.flush
  → [.\core25.ioLayer.lore:404] emit: input.flush
  → [.\core25.ioLayer.lore:54] trigger: input.flush
  → [.\core25.ioLayer.lore:404] emit: input.flush

🚨 Infinite loop detected on signal: 'quantum.branch.resolved'
  → [.\core.kernel.lore:21] trigger: quantum.branch.resolved
  → [.\core.kernel.lore:557] emit: quantum.branch.resolved
  → [.\core9.quantumBranch.lore:26] trigger: quantum.branch.resolved
  → [.\core.kernel.lore:21] trigger: quantum.branch.resolved
  → [.\core.kernel.lore:557] emit: quantum.branch.resolved
  → [.\core9.quantumBranch.lore:26] trigger: quantum.branch.resolved

🚨 Infinite loop detected on signal: 'ctx.returnTo'
  → [.\core13.storyEngine.lore:378] emit: ctx.returnTo
  → [.\core13.storyEngine.lore:429] trigger: ctx.returnTo
  → [.\core13.storyEngine.lore:378] emit: ctx.returnTo
  → [.\core13.storyEngine.lore:429] trigger: ctx.returnTo

🚨 Infinite loop detected on signal: 'boot.capsuleExecutor'
  → [.\core11.capsuleExecutor.lore:5] trigger: boot.capsuleExecutor
  → [.\core11.capsuleExecutor.lore:150] emit: boot.capsuleExecutor
  → [.\core11.capsuleExecutor.lore:5] trigger: boot.capsuleExecutor
  → [.\core11.capsuleExecutor.lore:150] emit: boot.capsuleExecutor

🚨 Infinite loop detected on signal: 'capsule.lineage'
  → [.\core22.symbolicCompiler.lore:114] trigger: capsule.lineage
  → [.\core22.symbolicCompiler.lore:450] emit: capsule.lineage
  → [.\core22.symbolicCompiler.lore:114] trigger: capsule.lineage
  → [.\core22.symbolicCompiler.lore:450] emit: capsule.lineage

🚨 Infinite loop detected on signal: 'capsuleLoader.ready'
  → [.\core0.boot.lore:22] emit: capsuleLoader.ready
  → [.\core0.boot.lore:39] trigger: capsuleLoader.ready
  → [.\core0.boot.lore:44] when: capsuleLoader.ready
  → [.\core0.boot.lore:22] emit: capsuleLoader.ready
  → [.\core0.boot.lore:39] trigger: capsuleLoader.ready
  → [.\core0.boot.lore:44] when: capsuleLoader.ready

🚨 Infinite loop detected on signal: 'holoframe.load'
  → [.\core19.visualFlow.lore:209] trigger: holoframe.load
  → [.\core19.visualFlow.lore:404] emit: holoframe.load
  → [.\core19.visualFlow.lore:209] trigger: holoframe.load
  → [.\core19.visualFlow.lore:404] emit: holoframe.load

🚨 Infinite loop detected on signal: 'draw.fadeIn'
  → [.\core19.visualFlow.lore:189] trigger: draw.fadeIn
  → [.\core19.visualFlow.lore:474] emit: draw.fadeIn
  → [.\core19.visualFlow.lore:189] trigger: draw.fadeIn
  → [.\core19.visualFlow.lore:474] emit: draw.fadeIn

🚨 Infinite loop detected on signal: 'stack.clone'
  → [.\core15.executionFlow.lore:237] trigger: stack.clone
  → [.\core15.executionFlow.lore:392] emit: stack.clone
  → [.\core15.executionFlow.lore:237] trigger: stack.clone
  → [.\core15.executionFlow.lore:392] emit: stack.clone

🚨 Infinite loop detected on signal: 'entropy.measure'
  → [.\core.kernel.lore:53] trigger: entropy.measure
  → [.\core.kernel.lore:577] emit: entropy.measure
  → [.\core.kernel.lore:53] trigger: entropy.measure
  → [.\core.kernel.lore:577] emit: entropy.measure

🚨 Infinite loop detected on signal: 'map.validate'
  → [.\core21.navigationLayer.lore:41] trigger: map.validate
  → [.\core21.navigationLayer.lore:211] emit: map.validate
  → [.\core21.navigationLayer.lore:41] trigger: map.validate
  → [.\core21.navigationLayer.lore:211] emit: map.validate

🚨 Infinite loop detected on signal: 'resume.skip'
  → [.\core0.boot.lore:87] trigger: resume.skip
  → [.\core0.boot.lore:112] emit: resume.skip
  → [.\core0.boot.lore:87] trigger: resume.skip
  → [.\core0.boot.lore:112] emit: resume.skip

🚨 Infinite loop detected on signal: 'parse.capsules'
  → [.\core1.symbolicBoot.lore:159] trigger: parse.capsules
  → [.\core1.symbolicBoot.lore:447] emit: parse.capsules
  → [.\core1.symbolicBoot.lore:159] trigger: parse.capsules
  → [.\core1.symbolicBoot.lore:447] emit: parse.capsules

🚨 Infinite loop detected on signal: 'entity.memory.append'
  → [.\core14.contextReflector.lore:273] trigger: entity.memory.append
  → [.\core14.contextReflector.lore:381] emit: entity.memory.append
  → [.\core14.contextReflector.lore:273] trigger: entity.memory.append
  → [.\core14.contextReflector.lore:381] emit: entity.memory.append

🚨 Infinite loop detected on signal: 'grammar.crosslink.rules'
  → [.\core12.languageLayer.lore:99] emit: grammar.crosslink.rules
  → [.\core12.languageLayer.lore:197] trigger: grammar.crosslink.rules
  → [.\core12.languageLayer.lore:99] emit: grammar.crosslink.rules
  → [.\core12.languageLayer.lore:197] trigger: grammar.crosslink.rules

🚨 Infinite loop detected on signal: 'triggerEngine.loop'
  → [.\core3.triggerEvaluator.lore:105] trigger: triggerEngine.loop
  → [.\core3.triggerEvaluator.lore:166] emit: triggerEngine.loop
  → [.\core3.triggerEvaluator.lore:105] trigger: triggerEngine.loop
  → [.\core3.triggerEvaluator.lore:166] emit: triggerEngine.loop

🚨 Infinite loop detected on signal: 'self.reflect.identity'
  → [.\core10.reflectiveSelf.lore:14] trigger: self.reflect.identity
  → [.\core10.reflectiveSelf.lore:68] emit: self.reflect.identity
  → [.\core10.reflectiveSelf.lore:73] when: self.reflect.identity
  → [.\core10.reflectiveSelf.lore:14] trigger: self.reflect.identity
  → [.\core10.reflectiveSelf.lore:68] emit: self.reflect.identity
  → [.\core10.reflectiveSelf.lore:73] when: self.reflect.identity

🚨 Infinite loop detected on signal: 'runtime.boot'
  → [.\core0.boot.assembly.lore:11] emit: runtime.boot
  → [.\core0.boot.lore:56] emit: runtime.boot
  → [.\core1.symbolicBoot.lore:69] trigger: runtime.boot
  → [.\core1.symbolicBoot.lore:93] emit: runtime.boot
  → [.\core0.boot.assembly.lore:11] emit: runtime.boot
  → [.\core0.boot.lore:56] emit: runtime.boot
  → [.\core1.symbolicBoot.lore:69] trigger: runtime.boot
  → [.\core1.symbolicBoot.lore:93] emit: runtime.boot

🚨 Infinite loop detected on signal: 'cognition.loop'
  → [.\core6.echoCognition.lore:12] emit: cognition.loop
  → [.\core6.echoCognition.lore:46] trigger: cognition.loop
  → [.\core6.echoCognition.lore:83] emit: cognition.loop
  → [.\core6.echoCognition.lore:12] emit: cognition.loop
  → [.\core6.echoCognition.lore:46] trigger: cognition.loop
  → [.\core6.echoCognition.lore:83] emit: cognition.loop

🚨 Infinite loop detected on signal: 'entropy.regulate'
  → [.\core.kernel.lore:51] trigger: entropy.regulate
  → [.\core.kernel.lore:522] emit: entropy.regulate
  → [.\core.kernel.lore:51] trigger: entropy.regulate
  → [.\core.kernel.lore:522] emit: entropy.regulate

🚨 Infinite loop detected on signal: 'logic.merge'
  → [.\core17.choiceLogic.lore:54] emit: logic.merge
  → [.\core17.choiceLogic.lore:294] trigger: logic.merge
  → [.\core17.choiceLogic.lore:54] emit: logic.merge
  → [.\core17.choiceLogic.lore:294] trigger: logic.merge

🚨 Infinite loop detected on signal: 'move.random'
  → [.\core21.navigationLayer.lore:101] trigger: move.random
  → [.\core21.navigationLayer.lore:181] emit: move.random
  → [.\core21.navigationLayer.lore:101] trigger: move.random
  → [.\core21.navigationLayer.lore:181] emit: move.random

🚨 Infinite loop detected on signal: 'gravity.evaluate'
  → [.\core14.contextReflector.lore:160] emit: gravity.evaluate
  → [.\core14.contextReflector.lore:165] trigger: gravity.evaluate
  → [.\core14.contextReflector.lore:160] emit: gravity.evaluate
  → [.\core14.contextReflector.lore:165] trigger: gravity.evaluate

🚨 Infinite loop detected on signal: 'compile.capsule'
  → [.\core22.symbolicCompiler.lore:17] trigger: compile.capsule
  → [.\core22.symbolicCompiler.lore:470] emit: compile.capsule
  → [.\core22.symbolicCompiler.lore:17] trigger: compile.capsule
  → [.\core22.symbolicCompiler.lore:470] emit: compile.capsule

🚨 Infinite loop detected on signal: 'sensor.init'
  → [.\core25.ioLayer.lore:205] trigger: sensor.init
  → [.\core25.ioLayer.lore:379] emit: sensor.init
  → [.\core25.ioLayer.lore:205] trigger: sensor.init
  → [.\core25.ioLayer.lore:379] emit: sensor.init

🚨 Infinite loop detected on signal: 'draw.3d.clear'
  → [.\core19.visualFlow.lore:383] trigger: draw.3d.clear
  → [.\core19.visualFlow.lore:484] emit: draw.3d.clear
  → [.\core19.visualFlow.lore:383] trigger: draw.3d.clear
  → [.\core19.visualFlow.lore:484] emit: draw.3d.clear

🚨 Infinite loop detected on signal: 'tutorial.glyph.structural'
  → [.\glyphTutorial.lore:14] emit: tutorial.glyph.structural
  → [.\glyphTutorial.lore:25] trigger: tutorial.glyph.structural
  → [.\glyphTutorial.lore:14] emit: tutorial.glyph.structural
  → [.\glyphTutorial.lore:25] trigger: tutorial.glyph.structural

🚨 Infinite loop detected on signal: 'boot.echoCognition'
  → [.\core6.echoCognition.lore:5] trigger: boot.echoCognition
  → [.\core6.echoCognition.lore:364] emit: boot.echoCognition
  → [.\core6.echoCognition.lore:5] trigger: boot.echoCognition
  → [.\core6.echoCognition.lore:364] emit: boot.echoCognition

🚨 Infinite loop detected on signal: 'compile.abort'
  → [.\core22.symbolicCompiler.lore:67] trigger: compile.abort
  → [.\core22.symbolicCompiler.lore:440] emit: compile.abort
  → [.\core22.symbolicCompiler.lore:67] trigger: compile.abort
  → [.\core22.symbolicCompiler.lore:440] emit: compile.abort

🚨 Infinite loop detected on signal: 'clock.start'
  → [.\core15.executionFlow.lore:254] trigger: clock.start
  → [.\core15.executionFlow.lore:327] emit: clock.start
  → [.\core15.executionFlow.lore:254] trigger: clock.start
  → [.\core15.executionFlow.lore:327] emit: clock.start

🚨 Infinite loop detected on signal: 'capsule.package.load'
  → [.\core22.symbolicCompiler.lore:214] emit: capsule.package.load
  → [.\core22.symbolicCompiler.lore:221] trigger: capsule.package.load
  → [.\core22.symbolicCompiler.lore:214] emit: capsule.package.load
  → [.\core22.symbolicCompiler.lore:221] trigger: capsule.package.load

🚨 Infinite loop detected on signal: 'dream.end'
  → [.\core24.symbolicCognition.lore:181] trigger: dream.end
  → [.\core24.symbolicCognition.lore:356] emit: dream.end
  → [.\core24.symbolicCognition.lore:181] trigger: dream.end
  → [.\core24.symbolicCognition.lore:356] emit: dream.end

🚨 Infinite loop detected on signal: 'quantum.mesh.link'
  → [.\core.kernel.lore:58] trigger: quantum.mesh.link
  → [.\core.kernel.lore:607] emit: quantum.mesh.link
  → [.\core.kernel.lore:58] trigger: quantum.mesh.link
  → [.\core.kernel.lore:607] emit: quantum.mesh.link

🚨 Infinite loop detected on signal: 'output.feedback.annotate'
  → [.\core25.ioLayer.lore:196] trigger: output.feedback.annotate
  → [.\core25.ioLayer.lore:414] emit: output.feedback.annotate
  → [.\core25.ioLayer.lore:196] trigger: output.feedback.annotate
  → [.\core25.ioLayer.lore:414] emit: output.feedback.annotate

🚨 Infinite loop detected on signal: 'exit.symbolic'
  → [.\core15.executionFlow.lore:180] emit: exit.symbolic
  → [.\core15.executionFlow.lore:278] trigger: exit.symbolic
  → [.\core15.executionFlow.lore:180] emit: exit.symbolic
  → [.\core15.executionFlow.lore:278] trigger: exit.symbolic

🚨 Infinite loop detected on signal: 'context.pushed'
  → [.\core0.boot.lore:162] trigger: context.pushed
  → [.\core0.boot.lore:170] emit: context.pushed
  → [.\core0.boot.lore:162] trigger: context.pushed
  → [.\core0.boot.lore:170] emit: context.pushed

🚨 Infinite loop detected on signal: 'story.continue'
  → [.\core13.storyEngine.lore:183] trigger: story.continue
  → [.\core13.storyEngine.lore:241] when: story.continue
  → [.\core13.storyEngine.lore:509] emit: story.continue
  → [.\core13.storyEngine.lore:183] trigger: story.continue
  → [.\core13.storyEngine.lore:241] when: story.continue
  → [.\core13.storyEngine.lore:509] emit: story.continue

🚨 Infinite loop detected on signal: 'signalInspector.observe.start'
  → [.\core16.signalInspector.lore:224] emit: signalInspector.observe.start
  → [.\core16.signalInspector.lore:229] trigger: signalInspector.observe.start
  → [.\core16.signalInspector.lore:224] emit: signalInspector.observe.start
  → [.\core16.signalInspector.lore:229] trigger: signalInspector.observe.start

🚨 Infinite loop detected on signal: 'grammar.rollback'
  → [.\core22.symbolicCompiler.lore:257] trigger: grammar.rollback
  → [.\core22.symbolicCompiler.lore:395] emit: grammar.rollback
  → [.\core22.symbolicCompiler.lore:257] trigger: grammar.rollback
  → [.\core22.symbolicCompiler.lore:395] emit: grammar.rollback

🚨 Infinite loop detected on signal: 'entity.memory.snapshot'
  → [.\core14.contextReflector.lore:281] trigger: entity.memory.snapshot
  → [.\core14.contextReflector.lore:456] emit: entity.memory.snapshot
  → [.\core14.contextReflector.lore:281] trigger: entity.memory.snapshot
  → [.\core14.contextReflector.lore:456] emit: entity.memory.snapshot

🚨 Infinite loop detected on signal: 'learn.pattern'
  → [.\core24.symbolicCognition.lore:5] trigger: learn.pattern
  → [.\core24.symbolicCognition.lore:396] emit: learn.pattern
  → [.\core24.symbolicCognition.lore:5] trigger: learn.pattern
  → [.\core24.symbolicCognition.lore:396] emit: learn.pattern

🚨 Infinite loop detected on signal: 'quantumBranch.resolve'
  → [.\core9.quantumBranch.lore:22] trigger: quantumBranch.resolve
  → [.\core9.quantumBranch.lore:142] emit: quantumBranch.resolve
  → [.\core9.quantumBranch.lore:22] trigger: quantumBranch.resolve
  → [.\core9.quantumBranch.lore:142] emit: quantumBranch.resolve

🚨 Infinite loop detected on signal: 'boot.reflectiveRuntime'
  → [.\core7.reflectiveRuntime.lore:3] trigger: boot.reflectiveRuntime
  → [.\core7.reflectiveRuntime.lore:171] emit: boot.reflectiveRuntime
  → [.\core7.reflectiveRuntime.lore:3] trigger: boot.reflectiveRuntime
  → [.\core7.reflectiveRuntime.lore:171] emit: boot.reflectiveRuntime

🚨 Infinite loop detected on signal: 'boot.ready'
  → [.\core.kernel.lore:11] trigger: boot.ready
  → [.\core.kernel.lore:537] emit: boot.ready
  → [.\core.kernel.lore:11] trigger: boot.ready
  → [.\core.kernel.lore:537] emit: boot.ready

🚨 Infinite loop detected on signal: 'web.dom.inject'
  → [.\core23.bridgeLayer.lore:155] emit: web.dom.inject
  → [.\core23.bridgeLayer.lore:160] trigger: web.dom.inject
  → [.\core23.bridgeLayer.lore:155] emit: web.dom.inject
  → [.\core23.bridgeLayer.lore:160] trigger: web.dom.inject

🚨 Infinite loop detected on signal: 'symbolicSubstrate.route.signal'
  → [.\core4.symbolicSubstrate.lore:191] trigger: symbolicSubstrate.route.signal
  → [.\core4.symbolicSubstrate.lore:468] emit: symbolicSubstrate.route.signal
  → [.\core4.symbolicSubstrate.lore:191] trigger: symbolicSubstrate.route.signal
  → [.\core4.symbolicSubstrate.lore:468] emit: symbolicSubstrate.route.signal

🚨 Infinite loop detected on signal: 'trace.recall'
  → [.\core16.signalInspector.lore:166] trigger: trace.recall
  → [.\core16.signalInspector.lore:356] emit: trace.recall
  → [.\core16.signalInspector.lore:166] trigger: trace.recall
  → [.\core16.signalInspector.lore:356] emit: trace.recall

🚨 Infinite loop detected on signal: 'buffer.clear.echo'
  → [.\core5.echoMesh.lore:292] trigger: buffer.clear.echo
  → [.\core5.echoMesh.lore:366] emit: buffer.clear.echo
  → [.\core5.echoMesh.lore:292] trigger: buffer.clear.echo
  → [.\core5.echoMesh.lore:366] emit: buffer.clear.echo

🚨 Infinite loop detected on signal: 'boot.quantumBranch'
  → [.\core9.quantumBranch.lore:10] trigger: boot.quantumBranch
  → [.\core9.quantumBranch.lore:222] emit: boot.quantumBranch
  → [.\core9.quantumBranch.lore:10] trigger: boot.quantumBranch
  → [.\core9.quantumBranch.lore:222] emit: boot.quantumBranch

🚨 Infinite loop detected on signal: 'grammar.fork'
  → [.\core22.symbolicCompiler.lore:268] trigger: grammar.fork
  → [.\core22.symbolicCompiler.lore:465] emit: grammar.fork
  → [.\core22.symbolicCompiler.lore:268] trigger: grammar.fork
  → [.\core22.symbolicCompiler.lore:465] emit: grammar.fork

🚨 Infinite loop detected on signal: 'capsule.snapshot'
  → [.\core7.reflectiveRuntime.lore:77] emit: capsule.snapshot
  → [.\core7.reflectiveRuntime.lore:84] trigger: capsule.snapshot
  → [.\core7.reflectiveRuntime.lore:77] emit: capsule.snapshot
  → [.\core7.reflectiveRuntime.lore:84] trigger: capsule.snapshot

🚨 Infinite loop detected on signal: 'kernel.reflect.init'
  → [.\core.kernel.lore:49] trigger: kernel.reflect.init
  → [.\core.kernel.lore:517] emit: kernel.reflect.init
  → [.\core.kernel.lore:49] trigger: kernel.reflect.init
  → [.\core.kernel.lore:517] emit: kernel.reflect.init

🚨 Infinite loop detected on signal: 'story.loop'
  → [.\core13.storyEngine.lore:11] emit: story.loop
  → [.\core13.storyEngine.lore:33] trigger: story.loop
  → [.\core13.storyEngine.lore:67] when: story.loop
  → [.\core13.storyEngine.lore:70] emit: story.loop
  → [.\core13.storyEngine.lore:11] emit: story.loop
  → [.\core13.storyEngine.lore:33] trigger: story.loop
  → [.\core13.storyEngine.lore:67] when: story.loop
  → [.\core13.storyEngine.lore:70] emit: story.loop

🚨 Infinite loop detected on signal: 'input.spellcheck'
  → [.\core25.ioLayer.lore:40] trigger: input.spellcheck
  → [.\core25.ioLayer.lore:94] emit: input.spellcheck
  → [.\core25.ioLayer.lore:40] trigger: input.spellcheck
  → [.\core25.ioLayer.lore:94] emit: input.spellcheck

🚨 Infinite loop detected on signal: 'symbolicSubstrate.resolve.memory'
  → [.\core.kernel.lore:215] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:8] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:27] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:108] trigger: symbolicSubstrate.resolve.memory
  → [.\core.kernel.lore:215] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:8] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:27] emit: symbolicSubstrate.resolve.memory
  → [.\core4.symbolicSubstrate.lore:108] trigger: symbolicSubstrate.resolve.memory

🚨 Infinite loop detected on signal: 'context.push'
  → [.\core0.boot.lore:23] emit: context.push
  → [.\core0.boot.lore:158] trigger: context.push
  → [.\core0.boot.lore:23] emit: context.push
  → [.\core0.boot.lore:158] trigger: context.push

🚨 Infinite loop detected on signal: 'context.trace.view'
  → [.\core14.contextReflector.lore:74] trigger: context.trace.view
  → [.\core14.contextReflector.lore:451] emit: context.trace.view
  → [.\core14.contextReflector.lore:74] trigger: context.trace.view
  → [.\core14.contextReflector.lore:451] emit: context.trace.view

🚨 Infinite loop detected on signal: 'grammar.snapshot'
  → [.\core22.symbolicCompiler.lore:246] trigger: grammar.snapshot
  → [.\core22.symbolicCompiler.lore:425] emit: grammar.snapshot
  → [.\core22.symbolicCompiler.lore:246] trigger: grammar.snapshot
  → [.\core22.symbolicCompiler.lore:425] emit: grammar.snapshot

🚨 Infinite loop detected on signal: 'pulse.inject.echo'
  → [.\core5.echoMesh.lore:283] trigger: pulse.inject.echo
  → [.\core5.echoMesh.lore:326] emit: pulse.inject.echo
  → [.\core5.echoMesh.lore:283] trigger: pulse.inject.echo
  → [.\core5.echoMesh.lore:326] emit: pulse.inject.echo

🚨 Infinite loop detected on signal: 'handlerRegistry.scan'
  → [.\core4.symbolicSubstrate.lore:93] emit: handlerRegistry.scan
  → [.\core4.symbolicSubstrate.lore:439] trigger: handlerRegistry.scan
  → [.\core4.symbolicSubstrate.lore:93] emit: handlerRegistry.scan
  → [.\core4.symbolicSubstrate.lore:439] trigger: handlerRegistry.scan

🚨 Infinite loop detected on signal: 'echo.pulse'
  → [.\core6.echoCognition.lore:258] trigger: echo.pulse
  → [.\core6.echoCognition.lore:394] emit: echo.pulse
  → [.\core6.echoCognition.lore:258] trigger: echo.pulse
  → [.\core6.echoCognition.lore:394] emit: echo.pulse

🚨 Infinite loop detected on signal: 'ctx.onTrigger'
  → [.\core18.emotiveLayer.lore:246] emit: ctx.onTrigger
  → [.\core18.emotiveLayer.lore:301] trigger: ctx.onTrigger
  → [.\core18.emotiveLayer.lore:246] emit: ctx.onTrigger
  → [.\core18.emotiveLayer.lore:301] trigger: ctx.onTrigger

🚨 Infinite loop detected on signal: 'choice.resolved'
  → [.\core17.choiceLogic.lore:9] emit: choice.resolved
  → [.\core17.choiceLogic.lore:21] emit: choice.resolved
  → [.\core17.choiceLogic.lore:37] emit: choice.resolved
  → [.\core17.choiceLogic.lore:45] emit: choice.resolved
  → [.\core17.choiceLogic.lore:282] trigger: choice.resolved
  → [.\core17.choiceLogic.lore:9] emit: choice.resolved
  → [.\core17.choiceLogic.lore:21] emit: choice.resolved
  → [.\core17.choiceLogic.lore:37] emit: choice.resolved
  → [.\core17.choiceLogic.lore:45] emit: choice.resolved
  → [.\core17.choiceLogic.lore:282] trigger: choice.resolved

🚨 Infinite loop detected on signal: 'holoframe.render'
  → [.\core19.visualFlow.lore:214] emit: holoframe.render
  → [.\core19.visualFlow.lore:221] trigger: holoframe.render
  → [.\core19.visualFlow.lore:214] emit: holoframe.render
  → [.\core19.visualFlow.lore:221] trigger: holoframe.render

🚨 Infinite loop detected on signal: 'entity.spawn'
  → [.\core20.symbolicEntity.lore:5] trigger: entity.spawn
  → [.\core20.symbolicEntity.lore:300] emit: entity.spawn
  → [.\core20.symbolicEntity.lore:5] trigger: entity.spawn
  → [.\core20.symbolicEntity.lore:300] emit: entity.spawn

🚨 Infinite loop detected on signal: 'audio.volumeFade'
  → [.\core18.emotiveLayer.lore:35] emit: audio.volumeFade
  → [.\core18.emotiveLayer.lore:45] emit: audio.volumeFade
  → [.\core18.emotiveLayer.lore:86] trigger: audio.volumeFade
  → [.\core18.emotiveLayer.lore:35] emit: audio.volumeFade
  → [.\core18.emotiveLayer.lore:45] emit: audio.volumeFade
  → [.\core18.emotiveLayer.lore:86] trigger: audio.volumeFade

🚨 Infinite loop detected on signal: 'match.signalPattern'
  → [.\core24.symbolicCognition.lore:57] trigger: match.signalPattern
  → [.\core24.symbolicCognition.lore:416] emit: match.signalPattern
  → [.\core24.symbolicCognition.lore:57] trigger: match.signalPattern
  → [.\core24.symbolicCognition.lore:416] emit: match.signalPattern

🚨 Infinite loop detected on signal: 'resume.complete'
  → [.\core0.boot.lore:86] trigger: resume.complete
  → [.\core0.boot.lore:105] emit: resume.complete
  → [.\core0.boot.lore:86] trigger: resume.complete
  → [.\core0.boot.lore:105] emit: resume.complete

🚨 Infinite loop detected on signal: 'story.memory'
  → [.\core13.storyEngine.lore:178] trigger: story.memory
  → [.\core13.storyEngine.lore:216] when: story.memory
  → [.\core13.storyEngine.lore:544] emit: story.memory
  → [.\core13.storyEngine.lore:178] trigger: story.memory
  → [.\core13.storyEngine.lore:216] when: story.memory
  → [.\core13.storyEngine.lore:544] emit: story.memory

🚨 Infinite loop detected on signal: 'ctx.onFail'
  → [.\core14.contextReflector.lore:127] emit: ctx.onFail
  → [.\core14.contextReflector.lore:363] trigger: ctx.onFail
  → [.\core24.symbolicCognition.lore:213] emit: ctx.onFail
  → [.\core14.contextReflector.lore:127] emit: ctx.onFail
  → [.\core14.contextReflector.lore:363] trigger: ctx.onFail
  → [.\core24.symbolicCognition.lore:213] emit: ctx.onFail

🚨 Infinite loop detected on signal: 'capsule.sandbox'
  → [.\core.kernel.lore:40] trigger: capsule.sandbox
  → [.\core.kernel.lore:507] emit: capsule.sandbox
  → [.\core.kernel.lore:40] trigger: capsule.sandbox
  → [.\core.kernel.lore:507] emit: capsule.sandbox

🚨 Infinite loop detected on signal: 'signal.received'
  → [.\core16.signalInspector.lore:74] emit: signal.received
  → [.\core16.signalInspector.lore:242] trigger: signal.received
  → [.\core16.signalInspector.lore:74] emit: signal.received
  → [.\core16.signalInspector.lore:242] trigger: signal.received

🚨 Infinite loop detected on signal: 'signal.rewrite'
  → [.\core16.signalInspector.lore:79] trigger: signal.rewrite
  → [.\core16.signalInspector.lore:406] emit: signal.rewrite
  → [.\core16.signalInspector.lore:79] trigger: signal.rewrite
  → [.\core16.signalInspector.lore:406] emit: signal.rewrite

🚨 Infinite loop detected on signal: 'decodeRequest'
  → [.\core0.boot.lore:247] emit: decodeRequest
  → [.\core0.boot.lore:676] trigger: decodeRequest
  → [.\core0.boot.lore:247] emit: decodeRequest
  → [.\core0.boot.lore:676] trigger: decodeRequest

🚨 Infinite loop detected on signal: 'loop.check'
  → [.\core15.executionFlow.lore:158] emit: loop.check
  → [.\core15.executionFlow.lore:165] trigger: loop.check
  → [.\core15.executionFlow.lore:158] emit: loop.check
  → [.\core15.executionFlow.lore:165] trigger: loop.check

🚨 Infinite loop detected on signal: 'web.ready'
  → [.\core23.bridgeLayer.lore:152] trigger: web.ready
  → [.\core23.bridgeLayer.lore:305] emit: web.ready
  → [.\core23.bridgeLayer.lore:152] trigger: web.ready
  → [.\core23.bridgeLayer.lore:305] emit: web.ready

🚨 Infinite loop detected on signal: 'network.receive'
  → [.\core23.bridgeLayer.lore:99] trigger: network.receive
  → [.\core23.bridgeLayer.lore:485] emit: network.receive
  → [.\core23.bridgeLayer.lore:99] trigger: network.receive
  → [.\core23.bridgeLayer.lore:485] emit: network.receive

🚨 Infinite loop detected on signal: 'visualFlow.boot'
  → [.\core19.visualFlow.lore:5] trigger: visualFlow.boot
  → [.\core19.visualFlow.lore:464] emit: visualFlow.boot
  → [.\core19.visualFlow.lore:5] trigger: visualFlow.boot
  → [.\core19.visualFlow.lore:464] emit: visualFlow.boot

🚨 Infinite loop detected on signal: 'map.visualize'
  → [.\core21.navigationLayer.lore:77] trigger: map.visualize
  → [.\core21.navigationLayer.lore:231] emit: map.visualize
  → [.\core21.navigationLayer.lore:77] trigger: map.visualize
  → [.\core21.navigationLayer.lore:231] emit: map.visualize

🚨 Infinite loop detected on signal: 'self.debug'
  → [.\core10.reflectiveSelf.lore:25] trigger: self.debug
  → [.\core10.reflectiveSelf.lore:139] when: self.debug
  → [.\core10.reflectiveSelf.lore:170] emit: self.debug
  → [.\core10.reflectiveSelf.lore:25] trigger: self.debug
  → [.\core10.reflectiveSelf.lore:139] when: self.debug
  → [.\core10.reflectiveSelf.lore:170] emit: self.debug

🚨 Infinite loop detected on signal: 'audio.loop'
  → [.\core18.emotiveLayer.lore:97] trigger: audio.loop
  → [.\core18.emotiveLayer.lore:370] emit: audio.loop
  → [.\core18.emotiveLayer.lore:97] trigger: audio.loop
  → [.\core18.emotiveLayer.lore:370] emit: audio.loop

🚨 Infinite loop detected on signal: 'signal.register'
  → [.\core16.signalInspector.lore:4] trigger: signal.register
  → [.\core16.signalInspector.lore:331] emit: signal.register
  → [.\core16.signalInspector.lore:4] trigger: signal.register
  → [.\core16.signalInspector.lore:331] emit: signal.register

🚨 Infinite loop detected on signal: 'self.reflect'
  → [.\core10.reflectiveSelf.lore:13] trigger: self.reflect
  → [.\core10.reflectiveSelf.lore:57] emit: self.reflect
  → [.\core10.reflectiveSelf.lore:62] when: self.reflect
  → [.\core10.reflectiveSelf.lore:13] trigger: self.reflect
  → [.\core10.reflectiveSelf.lore:57] emit: self.reflect
  → [.\core10.reflectiveSelf.lore:62] when: self.reflect

🚨 Infinite loop detected on signal: 'narrative.context.extract'
  → [.\core13.storyEngine.lore:345] trigger: narrative.context.extract
  → [.\core13.storyEngine.lore:372] when: narrative.context.extract
  → [.\core13.storyEngine.lore:594] emit: narrative.context.extract
  → [.\core13.storyEngine.lore:345] trigger: narrative.context.extract
  → [.\core13.storyEngine.lore:372] when: narrative.context.extract
  → [.\core13.storyEngine.lore:594] emit: narrative.context.extract

🚨 Infinite loop detected on signal: 'identity.matrix.align'
  → [.\core14.contextReflector.lore:334] emit: identity.matrix.align
  → [.\core14.contextReflector.lore:339] trigger: identity.matrix.align
  → [.\core14.contextReflector.lore:334] emit: identity.matrix.align
  → [.\core14.contextReflector.lore:339] trigger: identity.matrix.align

🚨 Infinite loop detected on signal: 'load.capsule'
  → [.\core2.capsuleEngine.lore:53] emit: load.capsule
  → [.\core2.capsuleEngine.lore:209] trigger: load.capsule
  → [.\core2.capsuleEngine.lore:53] emit: load.capsule
  → [.\core2.capsuleEngine.lore:209] trigger: load.capsule

🚨 Infinite loop detected on signal: 'web.listen.input'
  → [.\core0.boot.lore:556] trigger: web.listen.input
  → [.\core0.boot.lore:628] emit: web.listen.input
  → [.\core0.boot.lore:556] trigger: web.listen.input
  → [.\core0.boot.lore:628] emit: web.listen.input

🚨 Infinite loop detected on signal: 'capsule.package.import'
  → [.\core22.symbolicCompiler.lore:209] trigger: capsule.package.import
  → [.\core22.symbolicCompiler.lore:455] emit: capsule.package.import
  → [.\core22.symbolicCompiler.lore:209] trigger: capsule.package.import
  → [.\core22.symbolicCompiler.lore:455] emit: capsule.package.import

🚨 Infinite loop detected on signal: 'train.memory'
  → [.\core24.symbolicCognition.lore:26] trigger: train.memory
  → [.\core24.symbolicCognition.lore:381] emit: train.memory
  → [.\core24.symbolicCognition.lore:26] trigger: train.memory
  → [.\core24.symbolicCognition.lore:381] emit: train.memory

🚨 Infinite loop detected on signal: 'output.trace.log'
  → [.\core25.ioLayer.lore:172] emit: output.trace.log
  → [.\core25.ioLayer.lore:185] trigger: output.trace.log
  → [.\core25.ioLayer.lore:313] emit: output.trace.log
  → [.\core25.ioLayer.lore:172] emit: output.trace.log
  → [.\core25.ioLayer.lore:185] trigger: output.trace.log
  → [.\core25.ioLayer.lore:313] emit: output.trace.log

🚨 Infinite loop detected on signal: 'output.map'
  → [.\core25.ioLayer.lore:151] trigger: output.map
  → [.\core25.ioLayer.lore:349] emit: output.map
  → [.\core25.ioLayer.lore:151] trigger: output.map
  → [.\core25.ioLayer.lore:349] emit: output.map

🚨 Infinite loop detected on signal: 'triggerEngine.run'
  → [.\core3.triggerEvaluator.lore:77] emit: triggerEngine.run
  → [.\core3.triggerEvaluator.lore:86] trigger: triggerEngine.run
  → [.\core3.triggerEvaluator.lore:77] emit: triggerEngine.run
  → [.\core3.triggerEvaluator.lore:86] trigger: triggerEngine.run

🚨 Infinite loop detected on signal: 'input.listen'
  → [.\core25.ioLayer.lore:6] trigger: input.listen
  → [.\core25.ioLayer.lore:434] emit: input.listen
  → [.\core25.ioLayer.lore:6] trigger: input.listen
  → [.\core25.ioLayer.lore:434] emit: input.listen

🚨 Infinite loop detected on signal: 'substrate.reset.complete'
  → [.\core4.symbolicSubstrate.lore:301] emit: substrate.reset.complete
  → [.\core4.symbolicSubstrate.lore:325] trigger: substrate.reset.complete
  → [.\core4.symbolicSubstrate.lore:301] emit: substrate.reset.complete
  → [.\core4.symbolicSubstrate.lore:325] trigger: substrate.reset.complete

🚨 Infinite loop detected on signal: 'quantum.branch.finalize'
  → [.\core9.quantumBranch.lore:24] trigger: quantum.branch.finalize
  → [.\core9.quantumBranch.lore:150] emit: quantum.branch.finalize
  → [.\core9.quantumBranch.lore:24] trigger: quantum.branch.finalize
  → [.\core9.quantumBranch.lore:150] emit: quantum.branch.finalize

🚨 Infinite loop detected on signal: 'ghost.replay'
  → [.\core.kernel.lore:43] trigger: ghost.replay
  → [.\core.kernel.lore:527] emit: ghost.replay
  → [.\core.kernel.lore:43] trigger: ghost.replay
  → [.\core.kernel.lore:527] emit: ghost.replay

🚨 Infinite loop detected on signal: 'entity.memory.clear'
  → [.\core14.contextReflector.lore:265] trigger: entity.memory.clear
  → [.\core14.contextReflector.lore:396] emit: entity.memory.clear
  → [.\core14.contextReflector.lore:265] trigger: entity.memory.clear
  → [.\core14.contextReflector.lore:396] emit: entity.memory.clear

🚨 Infinite loop detected on signal: 'weave.begin'
  → [.\core13.storyEngine.lore:394] trigger: weave.begin
  → [.\core13.storyEngine.lore:403] when: weave.begin
  → [.\core13.storyEngine.lore:454] emit: weave.begin
  → [.\core13.storyEngine.lore:394] trigger: weave.begin
  → [.\core13.storyEngine.lore:403] when: weave.begin
  → [.\core13.storyEngine.lore:454] emit: weave.begin

🚨 Infinite loop detected on signal: 'resonance.transmit'
  → [.\core14.contextReflector.lore:218] emit: resonance.transmit
  → [.\core14.contextReflector.lore:223] trigger: resonance.transmit
  → [.\core14.contextReflector.lore:218] emit: resonance.transmit
  → [.\core14.contextReflector.lore:223] trigger: resonance.transmit

🚨 Infinite loop detected on signal: 'dna.activate'
  → [.\core20.symbolicEntity.lore:177] emit: dna.activate
  → [.\core20.symbolicEntity.lore:184] trigger: dna.activate
  → [.\core20.symbolicEntity.lore:177] emit: dna.activate
  → [.\core20.symbolicEntity.lore:184] trigger: dna.activate

🚨 Infinite loop detected on signal: 'gravity.pulse'
  → [.\core14.contextReflector.lore:157] trigger: gravity.pulse
  → [.\core14.contextReflector.lore:411] emit: gravity.pulse
  → [.\core14.contextReflector.lore:157] trigger: gravity.pulse
  → [.\core14.contextReflector.lore:411] emit: gravity.pulse

🚨 Infinite loop detected on signal: 'context.frame.restored'
  → [.\core0.boot.lore:165] trigger: context.frame.restored
  → [.\core0.boot.lore:195] emit: context.frame.restored
  → [.\core0.boot.lore:165] trigger: context.frame.restored
  → [.\core0.boot.lore:195] emit: context.frame.restored

🚨 Infinite loop detected on signal: 'self.snapshot'
  → [.\core10.reflectiveSelf.lore:21] trigger: self.snapshot
  → [.\core10.reflectiveSelf.lore:112] when: self.snapshot
  → [.\core10.reflectiveSelf.lore:185] emit: self.snapshot
  → [.\core10.reflectiveSelf.lore:21] trigger: self.snapshot
  → [.\core10.reflectiveSelf.lore:112] when: self.snapshot
  → [.\core10.reflectiveSelf.lore:185] emit: self.snapshot

🚨 Infinite loop detected on signal: 'device.capture.fail'
  → [.\core23.bridgeLayer.lore:320] emit: device.capture.fail
  → [.\core23.bridgeLayer.lore:336] trigger: device.capture.fail
  → [.\core23.bridgeLayer.lore:320] emit: device.capture.fail
  → [.\core23.bridgeLayer.lore:336] trigger: device.capture.fail

🚨 Infinite loop detected on signal: 'causality.trace.step'
  → [.\core13.storyEngine.lore:280] trigger: causality.trace.step
  → [.\core13.storyEngine.lore:312] emit: causality.trace.step
  → [.\core13.storyEngine.lore:314] when: causality.trace.step
  → [.\core13.storyEngine.lore:280] trigger: causality.trace.step
  → [.\core13.storyEngine.lore:312] emit: causality.trace.step
  → [.\core13.storyEngine.lore:314] when: causality.trace.step

🚨 Infinite loop detected on signal: 'capsule.fingerprint'
  → [.\core0.boot.lore:328] emit: capsule.fingerprint
  → [.\core1.symbolicBoot.lore:136] trigger: capsule.fingerprint
  → [.\core1.symbolicBoot.lore:170] trigger: capsule.fingerprint
  → [.\core22.symbolicCompiler.lore:103] trigger: capsule.fingerprint
  → [.\core0.boot.lore:328] emit: capsule.fingerprint
  → [.\core1.symbolicBoot.lore:136] trigger: capsule.fingerprint
  → [.\core1.symbolicBoot.lore:170] trigger: capsule.fingerprint
  → [.\core22.symbolicCompiler.lore:103] trigger: capsule.fingerprint

🚨 Infinite loop detected on signal: 'story.trace.lineage'
  → [.\core0.boot.lore:648] emit: story.trace.lineage
  → [.\core0.boot.lore:712] trigger: story.trace.lineage
  → [.\core0.boot.lore:648] emit: story.trace.lineage
  → [.\core0.boot.lore:712] trigger: story.trace.lineage

🚨 Infinite loop detected on signal: 'sensor.overlay'
  → [.\core25.ioLayer.lore:279] emit: sensor.overlay
  → [.\core25.ioLayer.lore:284] trigger: sensor.overlay
  → [.\core25.ioLayer.lore:279] emit: sensor.overlay
  → [.\core25.ioLayer.lore:284] trigger: sensor.overlay

🚨 Infinite loop detected on signal: 'self.observe.structure'
  → [.\core10.reflectiveSelf.lore:17] trigger: self.observe.structure
  → [.\core10.reflectiveSelf.lore:64] emit: self.observe.structure
  → [.\core10.reflectiveSelf.lore:89] when: self.observe.structure
  → [.\core10.reflectiveSelf.lore:17] trigger: self.observe.structure
  → [.\core10.reflectiveSelf.lore:64] emit: self.observe.structure
  → [.\core10.reflectiveSelf.lore:89] when: self.observe.structure

🚨 Infinite loop detected on signal: 'choice.debug'
  → [.\core17.choiceLogic.lore:244] trigger: choice.debug
  → [.\core17.choiceLogic.lore:307] emit: choice.debug
  → [.\core17.choiceLogic.lore:244] trigger: choice.debug
  → [.\core17.choiceLogic.lore:307] emit: choice.debug

🚨 Infinite loop detected on signal: 'causality.visualize'
  → [.\core13.storyEngine.lore:281] trigger: causality.visualize
  → [.\core13.storyEngine.lore:322] when: causality.visualize
  → [.\core13.storyEngine.lore:484] emit: causality.visualize
  → [.\core13.storyEngine.lore:281] trigger: causality.visualize
  → [.\core13.storyEngine.lore:322] when: causality.visualize
  → [.\core13.storyEngine.lore:484] emit: causality.visualize

🚨 Infinite loop detected on signal: 'device.gps.locate'
  → [.\core23.bridgeLayer.lore:353] trigger: device.gps.locate
  → [.\core23.bridgeLayer.lore:490] emit: device.gps.locate
  → [.\core23.bridgeLayer.lore:353] trigger: device.gps.locate
  → [.\core23.bridgeLayer.lore:490] emit: device.gps.locate

🚨 Infinite loop detected on signal: 'self.observe.memory'
  → [.\core10.reflectiveSelf.lore:18] trigger: self.observe.memory
  → [.\core10.reflectiveSelf.lore:65] emit: self.observe.memory
  → [.\core10.reflectiveSelf.lore:95] when: self.observe.memory
  → [.\core10.reflectiveSelf.lore:18] trigger: self.observe.memory
  → [.\core10.reflectiveSelf.lore:65] emit: self.observe.memory
  → [.\core10.reflectiveSelf.lore:95] when: self.observe.memory

🚨 Infinite loop detected on signal: 'device.storage.read'
  → [.\core23.bridgeLayer.lore:369] trigger: device.storage.read
  → [.\core23.bridgeLayer.lore:475] emit: device.storage.read
  → [.\core23.bridgeLayer.lore:369] trigger: device.storage.read
  → [.\core23.bridgeLayer.lore:475] emit: device.storage.read

🚨 Infinite loop detected on signal: 'input.map'
  → [.\core25.ioLayer.lore:24] trigger: input.map
  → [.\core25.ioLayer.lore:384] emit: input.map
  → [.\core25.ioLayer.lore:24] trigger: input.map
  → [.\core25.ioLayer.lore:384] emit: input.map

🚨 Infinite loop detected on signal: 'signal.incoming'
  → [.\core.kernel.lore:15] trigger: signal.incoming
  → [.\core.kernel.lore:597] emit: signal.incoming
  → [.\core.kernel.lore:15] trigger: signal.incoming
  → [.\core.kernel.lore:597] emit: signal.incoming

🚨 Infinite loop detected on signal: 'parse.embedded'
  → [.\core1.symbolicBoot.lore:161] trigger: parse.embedded
  → [.\core1.symbolicBoot.lore:427] emit: parse.embedded
  → [.\core1.symbolicBoot.lore:161] trigger: parse.embedded
  → [.\core1.symbolicBoot.lore:427] emit: parse.embedded

🚨 Infinite loop detected on signal: 'fertility.propagate'
  → [.\core24.symbolicCognition.lore:322] trigger: fertility.propagate
  → [.\core24.symbolicCognition.lore:351] emit: fertility.propagate
  → [.\core24.symbolicCognition.lore:322] trigger: fertility.propagate
  → [.\core24.symbolicCognition.lore:351] emit: fertility.propagate

🚨 Infinite loop detected on signal: 'visualFlow.ready'
  → [.\core19.visualFlow.lore:32] emit: visualFlow.ready
  → [.\core19.visualFlow.lore:39] trigger: visualFlow.ready
  → [.\core19.visualFlow.lore:32] emit: visualFlow.ready
  → [.\core19.visualFlow.lore:39] trigger: visualFlow.ready

🚨 Infinite loop detected on signal: 'witness.review'
  → [.\core16.signalInspector.lore:188] trigger: witness.review
  → [.\core16.signalInspector.lore:381] emit: witness.review
  → [.\core16.signalInspector.lore:188] trigger: witness.review
  → [.\core16.signalInspector.lore:381] emit: witness.review

🚨 Infinite loop detected on signal: 'kernel.halt'
  → [.\core0.boot.lore:369] emit: kernel.halt
  → [.\core0.boot.lore:700] trigger: kernel.halt
  → [.\core1.symbolicBoot.lore:776] emit: kernel.halt
  → [.\core0.boot.lore:369] emit: kernel.halt
  → [.\core0.boot.lore:700] trigger: kernel.halt
  → [.\core1.symbolicBoot.lore:776] emit: kernel.halt

🚨 Infinite loop detected on signal: 'mirror.capture.signal'
  → [.\core14.contextReflector.lore:107] trigger: mirror.capture.signal
  → [.\core14.contextReflector.lore:121] emit: mirror.capture.signal
  → [.\core14.contextReflector.lore:107] trigger: mirror.capture.signal
  → [.\core14.contextReflector.lore:121] emit: mirror.capture.signal

🚨 Infinite loop detected on signal: 'story.init'
  → [.\core13.storyEngine.lore:10] emit: story.init
  → [.\core13.storyEngine.lore:32] trigger: story.init
  → [.\core13.storyEngine.lore:57] when: story.init
  → [.\core13.storyEngine.lore:10] emit: story.init
  → [.\core13.storyEngine.lore:32] trigger: story.init
  → [.\core13.storyEngine.lore:57] when: story.init

🚨 Infinite loop detected on signal: 'subconscious.reflect'
  → [.\core14.contextReflector.lore:314] trigger: subconscious.reflect
  → [.\core14.contextReflector.lore:431] emit: subconscious.reflect
  → [.\core14.contextReflector.lore:314] trigger: subconscious.reflect
  → [.\core14.contextReflector.lore:431] emit: subconscious.reflect

🚨 Infinite loop detected on signal: 'draw.shake'
  → [.\core19.visualFlow.lore:179] trigger: draw.shake
  → [.\core19.visualFlow.lore:479] emit: draw.shake
  → [.\core19.visualFlow.lore:179] trigger: draw.shake
  → [.\core19.visualFlow.lore:479] emit: draw.shake

🚨 Infinite loop detected on signal: 'subconscious.store'
  → [.\core14.contextReflector.lore:290] trigger: subconscious.store
  → [.\core14.contextReflector.lore:491] emit: subconscious.store
  → [.\core14.contextReflector.lore:290] trigger: subconscious.store
  → [.\core14.contextReflector.lore:491] emit: subconscious.store

🚨 Infinite loop detected on signal: 'echo.signal'
  → [.\core5.echoMesh.lore:82] trigger: echo.signal
  → [.\core5.echoMesh.lore:331] emit: echo.signal
  → [.\core5.echoMesh.lore:82] trigger: echo.signal
  → [.\core5.echoMesh.lore:331] emit: echo.signal

🚨 Infinite loop detected on signal: 'mirror.snapshot'
  → [.\core14.contextReflector.lore:149] trigger: mirror.snapshot
  → [.\core14.contextReflector.lore:401] emit: mirror.snapshot
  → [.\core14.contextReflector.lore:149] trigger: mirror.snapshot
  → [.\core14.contextReflector.lore:401] emit: mirror.snapshot

🚨 Infinite loop detected on signal: 'parse.metadata'
  → [.\core1.symbolicBoot.lore:160] trigger: parse.metadata
  → [.\core1.symbolicBoot.lore:442] emit: parse.metadata
  → [.\core1.symbolicBoot.lore:160] trigger: parse.metadata
  → [.\core1.symbolicBoot.lore:442] emit: parse.metadata

🚨 Infinite loop detected on signal: 'subconscious.flush'
  → [.\core14.contextReflector.lore:306] trigger: subconscious.flush
  → [.\core14.contextReflector.lore:461] emit: subconscious.flush
  → [.\core14.contextReflector.lore:306] trigger: subconscious.flush
  → [.\core14.contextReflector.lore:461] emit: subconscious.flush

🚨 Infinite loop detected on signal: 'waitForQuery'
  → [.\core0.boot.lore:241] emit: waitForQuery
  → [.\core0.boot.lore:724] trigger: waitForQuery
  → [.\core0.boot.lore:241] emit: waitForQuery
  → [.\core0.boot.lore:724] trigger: waitForQuery

🚨 Infinite loop detected on signal: 'boot.storyEngine'
  → [.\core13.storyEngine.lore:6] trigger: boot.storyEngine
  → [.\core13.storyEngine.lore:499] emit: boot.storyEngine
  → [.\core13.storyEngine.lore:6] trigger: boot.storyEngine
  → [.\core13.storyEngine.lore:499] emit: boot.storyEngine

🚨 Infinite loop detected on signal: 'compile.reflectMemory'
  → [.\core22.symbolicCompiler.lore:366] trigger: compile.reflectMemory
  → [.\core22.symbolicCompiler.lore:460] emit: compile.reflectMemory
  → [.\core22.symbolicCompiler.lore:366] trigger: compile.reflectMemory
  → [.\core22.symbolicCompiler.lore:460] emit: compile.reflectMemory

🚨 Infinite loop detected on signal: 'boot.agenticLayer'
  → [.\core8.agenticLayer.lore:10] trigger: boot.agenticLayer
  → [.\core8.agenticLayer.lore:34] when: boot.agenticLayer
  → [.\core8.agenticLayer.lore:178] emit: boot.agenticLayer
  → [.\core8.agenticLayer.lore:10] trigger: boot.agenticLayer
  → [.\core8.agenticLayer.lore:34] when: boot.agenticLayer
  → [.\core8.agenticLayer.lore:178] emit: boot.agenticLayer

🚨 Infinite loop detected on signal: 'websocket.listen'
  → [.\core23.bridgeLayer.lore:306] emit: websocket.listen
  → [.\core23.bridgeLayer.lore:413] trigger: websocket.listen
  → [.\core23.bridgeLayer.lore:306] emit: websocket.listen
  → [.\core23.bridgeLayer.lore:413] trigger: websocket.listen

🚨 Infinite loop detected on signal: 'story.reflect'
  → [.\core13.storyEngine.lore:182] trigger: story.reflect
  → [.\core13.storyEngine.lore:237] when: story.reflect
  → [.\core13.storyEngine.lore:539] emit: story.reflect
  → [.\core13.storyEngine.lore:182] trigger: story.reflect
  → [.\core13.storyEngine.lore:237] when: story.reflect
  → [.\core13.storyEngine.lore:539] emit: story.reflect

🚨 Infinite loop detected on signal: 'entity.act'
  → [.\core20.symbolicEntity.lore:71] trigger: entity.act
  → [.\core20.symbolicEntity.lore:280] emit: entity.act
  → [.\core20.symbolicEntity.lore:71] trigger: entity.act
  → [.\core20.symbolicEntity.lore:280] emit: entity.act

🚨 Infinite loop detected on signal: 'interaction.matrix.ready'
  → [.\core20.symbolicEntity.lore:121] emit: interaction.matrix.ready
  → [.\core20.symbolicEntity.lore:128] trigger: interaction.matrix.ready
  → [.\core20.symbolicEntity.lore:121] emit: interaction.matrix.ready
  → [.\core20.symbolicEntity.lore:128] trigger: interaction.matrix.ready

🚨 Infinite loop detected on signal: 'sensor.read.tactile'
  → [.\core25.ioLayer.lore:266] trigger: sensor.read.tactile
  → [.\core25.ioLayer.lore:339] emit: sensor.read.tactile
  → [.\core25.ioLayer.lore:266] trigger: sensor.read.tactile
  → [.\core25.ioLayer.lore:339] emit: sensor.read.tactile

🚨 Infinite loop detected on signal: 'echo.sync.quantum'
  → [.\core9.quantumBranch.lore:111] emit: echo.sync.quantum
  → [.\core9.quantumBranch.lore:199] trigger: echo.sync.quantum
  → [.\core9.quantumBranch.lore:111] emit: echo.sync.quantum
  → [.\core9.quantumBranch.lore:199] trigger: echo.sync.quantum

🚨 Infinite loop detected on signal: 'memory.snapshot.create'
  → [.\core26.symbolicMeta.lore:148] trigger: memory.snapshot.create
  → [.\core26.symbolicMeta.lore:246] emit: memory.snapshot.create
  → [.\core26.symbolicMeta.lore:148] trigger: memory.snapshot.create
  → [.\core26.symbolicMeta.lore:246] emit: memory.snapshot.create

🚨 Infinite loop detected on signal: 'web.websocket.send'
  → [.\core23.bridgeLayer.lore:274] trigger: web.websocket.send
  → [.\core23.bridgeLayer.lore:500] emit: web.websocket.send
  → [.\core23.bridgeLayer.lore:274] trigger: web.websocket.send
  → [.\core23.bridgeLayer.lore:500] emit: web.websocket.send

🚨 Infinite loop detected on signal: 'perf.throttle'
  → [.\core26.symbolicMeta.lore:83] trigger: perf.throttle
  → [.\core26.symbolicMeta.lore:216] emit: perf.throttle
  → [.\core26.symbolicMeta.lore:83] trigger: perf.throttle
  → [.\core26.symbolicMeta.lore:216] emit: perf.throttle

🚨 Infinite loop detected on signal: 'capsule.dependency.resolve'
  → [.\core22.symbolicCompiler.lore:133] trigger: capsule.dependency.resolve
  → [.\core22.symbolicCompiler.lore:410] emit: capsule.dependency.resolve
  → [.\core22.symbolicCompiler.lore:133] trigger: capsule.dependency.resolve
  → [.\core22.symbolicCompiler.lore:410] emit: capsule.dependency.resolve

🚨 Infinite loop detected on signal: 'identity.set'
  → [.\core6.echoCognition.lore:260] trigger: identity.set
  → [.\core6.echoCognition.lore:277] when: identity.set
  → [.\core6.echoCognition.lore:369] emit: identity.set
  → [.\core6.echoCognition.lore:260] trigger: identity.set
  → [.\core6.echoCognition.lore:277] when: identity.set
  → [.\core6.echoCognition.lore:369] emit: identity.set

🚨 Infinite loop detected on signal: 'loader.boot'
  → [.\core0.boot.lore:42] trigger: loader.boot
  → [.\core0.boot.lore:66] when: loader.boot
  → [.\core1.symbolicBoot.lore:26] emit: loader.boot
  → [.\core0.boot.lore:42] trigger: loader.boot
  → [.\core0.boot.lore:66] when: loader.boot
  → [.\core1.symbolicBoot.lore:26] emit: loader.boot

🚨 Infinite loop detected on signal: 'self.ready'
  → [.\core10.reflectiveSelf.lore:52] emit: self.ready
  → [.\core10.reflectiveSelf.lore:152] trigger: self.ready
  → [.\core10.reflectiveSelf.lore:52] emit: self.ready
  → [.\core10.reflectiveSelf.lore:152] trigger: self.ready

🚨 Infinite loop detected on signal: 'loop.tick'
  → [.\core15.executionFlow.lore:149] emit: loop.tick
  → [.\core15.executionFlow.lore:154] trigger: loop.tick
  → [.\core15.executionFlow.lore:160] emit: loop.tick
  → [.\core15.executionFlow.lore:149] emit: loop.tick
  → [.\core15.executionFlow.lore:154] trigger: loop.tick
  → [.\core15.executionFlow.lore:160] emit: loop.tick

🚨 Infinite loop detected on signal: 'grammar.compare'
  → [.\core22.symbolicCompiler.lore:290] trigger: grammar.compare
  → [.\core22.symbolicCompiler.lore:390] emit: grammar.compare
  → [.\core22.symbolicCompiler.lore:290] trigger: grammar.compare
  → [.\core22.symbolicCompiler.lore:390] emit: grammar.compare

🚨 Infinite loop detected on signal: 'context.replace'
  → [.\core26.symbolicMeta.lore:20] trigger: context.replace
  → [.\core26.symbolicMeta.lore:221] emit: context.replace
  → [.\core26.symbolicMeta.lore:20] trigger: context.replace
  → [.\core26.symbolicMeta.lore:221] emit: context.replace

🚨 Infinite loop detected on signal: 'signal.allow'
  → [.\core.kernel.lore:17] trigger: signal.allow
  → [.\core.kernel.lore:512] emit: signal.allow
  → [.\core.kernel.lore:17] trigger: signal.allow
  → [.\core.kernel.lore:512] emit: signal.allow

🚨 Infinite loop detected on signal: 'boot.start'
  → [.\core0.boot.assembly.lore:13] emit: boot.start
  → [.\core0.boot.lore:24] emit: boot.start
  → [.\core0.boot.lore:652] trigger: boot.start
  → [.\core0.boot.assembly.lore:13] emit: boot.start
  → [.\core0.boot.lore:24] emit: boot.start
  → [.\core0.boot.lore:652] trigger: boot.start

🚨 Infinite loop detected on signal: 'parse.multiple.blocks'
  → [.\core1.symbolicBoot.lore:157] trigger: parse.multiple.blocks
  → [.\core1.symbolicBoot.lore:236] emit: parse.multiple.blocks
  → [.\core1.symbolicBoot.lore:157] trigger: parse.multiple.blocks
  → [.\core1.symbolicBoot.lore:236] emit: parse.multiple.blocks

🚨 Infinite loop detected on signal: 'runtime.awaitInput'
  → [.\core1.symbolicBoot.lore:709] emit: runtime.awaitInput
  → [.\core1.symbolicBoot.lore:818] emit: runtime.awaitInput
  → [.\core1.symbolicBoot.lore:1122] trigger: runtime.awaitInput
  → [.\core1.symbolicBoot.lore:709] emit: runtime.awaitInput
  → [.\core1.symbolicBoot.lore:818] emit: runtime.awaitInput
  → [.\core1.symbolicBoot.lore:1122] trigger: runtime.awaitInput

🚨 Infinite loop detected on signal: 'web.fetch.get'
  → [.\core23.bridgeLayer.lore:192] trigger: web.fetch.get
  → [.\core23.bridgeLayer.lore:420] emit: web.fetch.get
  → [.\core23.bridgeLayer.lore:192] trigger: web.fetch.get
  → [.\core23.bridgeLayer.lore:420] emit: web.fetch.get

🚨 Infinite loop detected on signal: 'desire.evaluate'
  → [.\core6.echoCognition.lore:263] trigger: desire.evaluate
  → [.\core6.echoCognition.lore:274] emit: desire.evaluate
  → [.\core6.echoCognition.lore:304] when: desire.evaluate
  → [.\core6.echoCognition.lore:263] trigger: desire.evaluate
  → [.\core6.echoCognition.lore:274] emit: desire.evaluate
  → [.\core6.echoCognition.lore:304] when: desire.evaluate

🚨 Infinite loop detected on signal: 'agent.ready'
  → [.\core8.agenticLayer.lore:18] trigger: agent.ready
  → [.\core8.agenticLayer.lore:48] emit: agent.ready
  → [.\core8.agenticLayer.lore:51] when: agent.ready
  → [.\core8.agenticLayer.lore:18] trigger: agent.ready
  → [.\core8.agenticLayer.lore:48] emit: agent.ready
  → [.\core8.agenticLayer.lore:51] when: agent.ready

🚨 Infinite loop detected on signal: 'grammar.check.all'
  → [.\grammar.lore:13] emit: grammar.check.all
  → [.\grammar.lore:15] trigger: grammar.check.all
  → [.\grammar.lore:13] emit: grammar.check.all
  → [.\grammar.lore:15] trigger: grammar.check.all

🚨 Infinite loop detected on signal: 'parse.from.file'
  → [.\core1.symbolicBoot.lore:156] trigger: parse.from.file
  → [.\core1.symbolicBoot.lore:228] emit: parse.from.file
  → [.\core1.symbolicBoot.lore:156] trigger: parse.from.file
  → [.\core1.symbolicBoot.lore:228] emit: parse.from.file

🚨 Infinite loop detected on signal: 'clock.status'
  → [.\core15.executionFlow.lore:263] trigger: clock.status
  → [.\core15.executionFlow.lore:307] emit: clock.status
  → [.\core15.executionFlow.lore:263] trigger: clock.status
  → [.\core15.executionFlow.lore:307] emit: clock.status

🚨 Infinite loop detected on signal: 'capsule.search'
  → [.\core0.boot.lore:390] trigger: capsule.search
  → [.\core0.boot.lore:414] when: capsule.search
  → [.\core0.boot.lore:450] emit: capsule.search
  → [.\core1.symbolicBoot.lore:168] trigger: capsule.search
  → [.\core0.boot.lore:390] trigger: capsule.search
  → [.\core0.boot.lore:414] when: capsule.search
  → [.\core0.boot.lore:450] emit: capsule.search
  → [.\core1.symbolicBoot.lore:168] trigger: capsule.search

🚨 Infinite loop detected on signal: 'capsule.inferType'
  → [.\core22.symbolicCompiler.lore:146] trigger: capsule.inferType
  → [.\core22.symbolicCompiler.lore:420] emit: capsule.inferType
  → [.\core22.symbolicCompiler.lore:146] trigger: capsule.inferType
  → [.\core22.symbolicCompiler.lore:420] emit: capsule.inferType

🚨 Infinite loop detected on signal: 'timeline.frame.next'
  → [.\core19.visualFlow.lore:65] emit: timeline.frame.next
  → [.\core19.visualFlow.lore:83] trigger: timeline.frame.next
  → [.\core19.visualFlow.lore:65] emit: timeline.frame.next
  → [.\core19.visualFlow.lore:83] trigger: timeline.frame.next

🚨 Infinite loop detected on signal: 'subconscious.retrieve'
  → [.\core14.contextReflector.lore:298] trigger: subconscious.retrieve
  → [.\core14.contextReflector.lore:476] emit: subconscious.retrieve
  → [.\core14.contextReflector.lore:298] trigger: subconscious.retrieve
  → [.\core14.contextReflector.lore:476] emit: subconscious.retrieve

🚨 Infinite loop detected on signal: 'logic.resolve.handlers'
  → [.\core4.symbolicSubstrate.lore:90] trigger: logic.resolve.handlers
  → [.\core4.symbolicSubstrate.lore:498] emit: logic.resolve.handlers
  → [.\core4.symbolicSubstrate.lore:90] trigger: logic.resolve.handlers
  → [.\core4.symbolicSubstrate.lore:498] emit: logic.resolve.handlers

🚨 Infinite loop detected on signal: 'echo.recall'
  → [.\core6.echoCognition.lore:262] trigger: echo.recall
  → [.\core6.echoCognition.lore:299] when: echo.recall
  → [.\core6.echoCognition.lore:389] emit: echo.recall
  → [.\core6.echoCognition.lore:262] trigger: echo.recall
  → [.\core6.echoCognition.lore:299] when: echo.recall
  → [.\core6.echoCognition.lore:389] emit: echo.recall

🚨 Infinite loop detected on signal: 'choiceLogic.boot'
  → [.\core17.choiceLogic.lore:174] trigger: choiceLogic.boot
  → [.\core17.choiceLogic.lore:317] emit: choiceLogic.boot
  → [.\core17.choiceLogic.lore:174] trigger: choiceLogic.boot
  → [.\core17.choiceLogic.lore:317] emit: choiceLogic.boot

🚨 Infinite loop detected on signal: 'resonance.align'
  → [.\core14.contextReflector.lore:239] trigger: resonance.align
  → [.\core14.contextReflector.lore:466] emit: resonance.align
  → [.\core14.contextReflector.lore:239] trigger: resonance.align
  → [.\core14.contextReflector.lore:466] emit: resonance.align

🚨 Infinite loop detected on signal: 'shell.activate'
  → [.\core19.visualFlow.lore:287] trigger: shell.activate
  → [.\core19.visualFlow.lore:434] emit: shell.activate
  → [.\core19.visualFlow.lore:287] trigger: shell.activate
  → [.\core19.visualFlow.lore:434] emit: shell.activate

🚨 Infinite loop detected on signal: 'cause.define'
  → [.\core13.storyEngine.lore:283] trigger: cause.define
  → [.\core13.storyEngine.lore:332] when: cause.define
  → [.\core13.storyEngine.lore:579] emit: cause.define
  → [.\core13.storyEngine.lore:283] trigger: cause.define
  → [.\core13.storyEngine.lore:332] when: cause.define
  → [.\core13.storyEngine.lore:579] emit: cause.define

🚨 Infinite loop detected on signal: 'draw.3d.material'
  → [.\core19.visualFlow.lore:342] trigger: draw.3d.material
  → [.\core19.visualFlow.lore:509] emit: draw.3d.material
  → [.\core19.visualFlow.lore:342] trigger: draw.3d.material
  → [.\core19.visualFlow.lore:509] emit: draw.3d.material

🚨 Infinite loop detected on signal: 'scan.forCapsules'
  → [.\core1.symbolicBoot.lore:152] trigger: scan.forCapsules
  → [.\core1.symbolicBoot.lore:452] emit: scan.forCapsules
  → [.\core1.symbolicBoot.lore:152] trigger: scan.forCapsules
  → [.\core1.symbolicBoot.lore:452] emit: scan.forCapsules

🚨 Infinite loop detected on signal: 'cap.revoke'
  → [.\core.kernel.lore:39] trigger: cap.revoke
  → [.\core.kernel.lore:532] emit: cap.revoke
  → [.\core.kernel.lore:39] trigger: cap.revoke
  → [.\core.kernel.lore:532] emit: cap.revoke

🚨 Infinite loop detected on signal: 'narrative.record'
  → [.\core13.storyEngine.lore:119] trigger: narrative.record
  → [.\core13.storyEngine.lore:136] when: narrative.record
  → [.\core13.storyEngine.lore:569] emit: narrative.record
  → [.\core13.storyEngine.lore:119] trigger: narrative.record
  → [.\core13.storyEngine.lore:136] when: narrative.record
  → [.\core13.storyEngine.lore:569] emit: narrative.record

🚨 Infinite loop detected on signal: 'perf.monitor'
  → [.\core26.symbolicMeta.lore:61] emit: perf.monitor
  → [.\core26.symbolicMeta.lore:66] trigger: perf.monitor
  → [.\core26.symbolicMeta.lore:172] emit: perf.monitor
  → [.\core26.symbolicMeta.lore:61] emit: perf.monitor
  → [.\core26.symbolicMeta.lore:66] trigger: perf.monitor
  → [.\core26.symbolicMeta.lore:172] emit: perf.monitor

🚨 Infinite loop detected on signal: 'capsule.rewrite'
  → [.\core7.reflectiveRuntime.lore:109] trigger: capsule.rewrite
  → [.\core7.reflectiveRuntime.lore:161] emit: capsule.rewrite
  → [.\core7.reflectiveRuntime.lore:109] trigger: capsule.rewrite
  → [.\core7.reflectiveRuntime.lore:161] emit: capsule.rewrite

🚨 Infinite loop detected on signal: 'reason.suggestNext'
  → [.\core16.signalInspector.lore:142] trigger: reason.suggestNext
  → [.\core16.signalInspector.lore:366] emit: reason.suggestNext
  → [.\core16.signalInspector.lore:142] trigger: reason.suggestNext
  → [.\core16.signalInspector.lore:366] emit: reason.suggestNext

🚨 Infinite loop detected on signal: 'fs.parseRoot'
  → [.\core1.symbolicBoot.lore:164] trigger: fs.parseRoot
  → [.\core1.symbolicBoot.lore:176] emit: fs.parseRoot
  → [.\core1.symbolicBoot.lore:164] trigger: fs.parseRoot
  → [.\core1.symbolicBoot.lore:176] emit: fs.parseRoot

🚨 Infinite loop detected on signal: 'grammar.bootstrap'
  → [.\core12.languageLayer.lore:34] emit: grammar.bootstrap
  → [.\core12.languageLayer.lore:40] when: grammar.bootstrap
  → [.\core12.languageLayer.lore:223] trigger: grammar.bootstrap
  → [.\core12.languageLayer.lore:34] emit: grammar.bootstrap
  → [.\core12.languageLayer.lore:40] when: grammar.bootstrap
  → [.\core12.languageLayer.lore:223] trigger: grammar.bootstrap

🚨 Infinite loop detected on signal: 'audio.play'
  → [.\core18.emotiveLayer.lore:5] trigger: audio.play
  → [.\core18.emotiveLayer.lore:325] emit: audio.play
  → [.\core18.emotiveLayer.lore:5] trigger: audio.play
  → [.\core18.emotiveLayer.lore:325] emit: audio.play

🚨 Infinite loop detected on signal: 'web.draw.sync'
  → [.\core0.boot.lore:577] trigger: web.draw.sync
  → [.\core0.boot.lore:629] emit: web.draw.sync
  → [.\core0.boot.lore:577] trigger: web.draw.sync
  → [.\core0.boot.lore:629] emit: web.draw.sync

🚨 Infinite loop detected on signal: 'story.debug'
  → [.\core10.reflectiveSelf.lore:26] trigger: story.debug
  → [.\core10.reflectiveSelf.lore:145] when: story.debug
  → [.\core10.reflectiveSelf.lore:175] emit: story.debug
  → [.\core13.storyEngine.lore:38] trigger: story.debug
  → [.\core13.storyEngine.lore:101] when: story.debug
  → [.\core10.reflectiveSelf.lore:26] trigger: story.debug
  → [.\core10.reflectiveSelf.lore:145] when: story.debug
  → [.\core10.reflectiveSelf.lore:175] emit: story.debug
  → [.\core13.storyEngine.lore:38] trigger: story.debug
  → [.\core13.storyEngine.lore:101] when: story.debug

🚨 Infinite loop detected on signal: 'draw.blink'
  → [.\core19.visualFlow.lore:169] trigger: draw.blink
  → [.\core19.visualFlow.lore:459] emit: draw.blink
  → [.\core19.visualFlow.lore:169] trigger: draw.blink
  → [.\core19.visualFlow.lore:459] emit: draw.blink

🚨 Infinite loop detected on signal: 'entity.speak'
  → [.\core20.symbolicEntity.lore:18] trigger: entity.speak
  → [.\core20.symbolicEntity.lore:245] emit: entity.speak
  → [.\core20.symbolicEntity.lore:18] trigger: entity.speak
  → [.\core20.symbolicEntity.lore:245] emit: entity.speak

🚨 Infinite loop detected on signal: 'navigation.init'
  → [.\core21.navigationLayer.lore:5] trigger: navigation.init
  → [.\core21.navigationLayer.lore:191] emit: navigation.init
  → [.\core21.navigationLayer.lore:5] trigger: navigation.init
  → [.\core21.navigationLayer.lore:191] emit: navigation.init

🚨 Infinite loop detected on signal: 'mesh.ready'
  → [.\core5.echoMesh.lore:21] emit: mesh.ready
  → [.\core5.echoMesh.lore:26] trigger: mesh.ready
  → [.\core5.echoMesh.lore:21] emit: mesh.ready
  → [.\core5.echoMesh.lore:26] trigger: mesh.ready

🚨 Infinite loop detected on signal: 'lens.shift'
  → [.\core19.visualFlow.lore:276] trigger: lens.shift
  → [.\core19.visualFlow.lore:414] emit: lens.shift
  → [.\core19.visualFlow.lore:276] trigger: lens.shift
  → [.\core19.visualFlow.lore:414] emit: lens.shift

🚨 Infinite loop detected on signal: 'identity.resolve'
  → [.\core14.contextReflector.lore:322] trigger: identity.resolve
  → [.\core14.contextReflector.lore:446] emit: identity.resolve
  → [.\core14.contextReflector.lore:322] trigger: identity.resolve
  → [.\core14.contextReflector.lore:446] emit: identity.resolve

🚨 Infinite loop detected on signal: 'output.send'
  → [.\core25.ioLayer.lore:159] trigger: output.send
  → [.\core25.ioLayer.lore:374] emit: output.send
  → [.\core25.ioLayer.lore:159] trigger: output.send
  → [.\core25.ioLayer.lore:374] emit: output.send

🚨 Infinite loop detected on signal: 'identity.evaluate'
  → [.\core6.echoCognition.lore:61] emit: identity.evaluate
  → [.\core6.echoCognition.lore:79] emit: identity.evaluate
  → [.\core6.echoCognition.lore:136] trigger: identity.evaluate
  → [.\core6.echoCognition.lore:61] emit: identity.evaluate
  → [.\core6.echoCognition.lore:79] emit: identity.evaluate
  → [.\core6.echoCognition.lore:136] trigger: identity.evaluate

🚨 Infinite loop detected on signal: 'web.websocket.connect'
  → [.\core23.bridgeLayer.lore:264] trigger: web.websocket.connect
  → [.\core23.bridgeLayer.lore:425] emit: web.websocket.connect
  → [.\core23.bridgeLayer.lore:264] trigger: web.websocket.connect
  → [.\core23.bridgeLayer.lore:425] emit: web.websocket.connect

🚨 Infinite loop detected on signal: 'triggerEngine.execute'
  → [.\core3.triggerEvaluator.lore:45] emit: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:55] trigger: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:72] emit: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:112] emit: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:45] emit: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:55] trigger: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:72] emit: triggerEngine.execute
  → [.\core3.triggerEvaluator.lore:112] emit: triggerEngine.execute

🚨 Infinite loop detected on signal: 'echoCognition.init'
  → [.\core6.echoCognition.lore:11] emit: echoCognition.init
  → [.\core6.echoCognition.lore:20] trigger: echoCognition.init
  → [.\core6.echoCognition.lore:11] emit: echoCognition.init
  → [.\core6.echoCognition.lore:20] trigger: echoCognition.init

🚨 Infinite loop detected on signal: 'perf.tickDelayed'
  → [.\core26.symbolicMeta.lore:86] emit: perf.tickDelayed
  → [.\core26.symbolicMeta.lore:91] trigger: perf.tickDelayed
  → [.\core26.symbolicMeta.lore:103] emit: perf.tickDelayed
  → [.\core26.symbolicMeta.lore:86] emit: perf.tickDelayed
  → [.\core26.symbolicMeta.lore:91] trigger: perf.tickDelayed
  → [.\core26.symbolicMeta.lore:103] emit: perf.tickDelayed

🚨 Infinite loop detected on signal: 'input.snapshot'
  → [.\core25.ioLayer.lore:293] trigger: input.snapshot
  → [.\core25.ioLayer.lore:312] emit: input.snapshot
  → [.\core25.ioLayer.lore:293] trigger: input.snapshot
  → [.\core25.ioLayer.lore:312] emit: input.snapshot

🚨 Infinite loop detected on signal: 'security.evolution'
  → [.\core.kernel.lore:54] trigger: security.evolution
  → [.\core.kernel.lore:587] emit: security.evolution
  → [.\core.kernel.lore:54] trigger: security.evolution
  → [.\core.kernel.lore:587] emit: security.evolution

🚨 Infinite loop detected on signal: 'device.microphone.listen'
  → [.\core23.bridgeLayer.lore:343] trigger: device.microphone.listen
  → [.\core23.bridgeLayer.lore:495] emit: device.microphone.listen
  → [.\core23.bridgeLayer.lore:343] trigger: device.microphone.listen
  → [.\core23.bridgeLayer.lore:495] emit: device.microphone.listen

🚨 Infinite loop detected on signal: 'input.meta.extract'
  → [.\core25.ioLayer.lore:104] emit: input.meta.extract
  → [.\core25.ioLayer.lore:119] trigger: input.meta.extract
  → [.\core25.ioLayer.lore:104] emit: input.meta.extract
  → [.\core25.ioLayer.lore:119] trigger: input.meta.extract

🚨 Infinite loop detected on signal: 'input.process'
  → [.\core25.ioLayer.lore:95] emit: input.process
  → [.\core25.ioLayer.lore:100] trigger: input.process
  → [.\core25.ioLayer.lore:95] emit: input.process
  → [.\core25.ioLayer.lore:100] trigger: input.process

🚨 Infinite loop detected on signal: 'cognition.glyph.init'
  → [.\core.kernel.lore:48] trigger: cognition.glyph.init
  → [.\core.kernel.lore:126] emit: cognition.glyph.init
  → [.\core.kernel.lore:48] trigger: cognition.glyph.init
  → [.\core.kernel.lore:126] emit: cognition.glyph.init

🚨 Infinite loop detected on signal: 'kernel.boot.checkpoint'
  → [.\core1.symbolicBoot.lore:875] emit: kernel.boot.checkpoint
  → [.\core1.symbolicBoot.lore:889] trigger: kernel.boot.checkpoint
  → [.\core1.symbolicBoot.lore:875] emit: kernel.boot.checkpoint
  → [.\core1.symbolicBoot.lore:889] trigger: kernel.boot.checkpoint

🚨 Infinite loop detected on signal: 'env.scan.root'
  → [.\core1.symbolicBoot.lore:18] emit: env.scan.root
  → [.\core1.symbolicBoot.lore:348] trigger: env.scan.root
  → [.\core1.symbolicBoot.lore:18] emit: env.scan.root
  → [.\core1.symbolicBoot.lore:348] trigger: env.scan.root

🚨 Infinite loop detected on signal: 'pulse.tick'
  → [.\core.kernel.lore:12] trigger: pulse.tick
  → [.\core.kernel.lore:567] emit: pulse.tick
  → [.\core.kernel.lore:12] trigger: pulse.tick
  → [.\core.kernel.lore:567] emit: pulse.tick

🚨 Infinite loop detected on signal: 'causality.explain'
  → [.\core13.storyEngine.lore:278] trigger: causality.explain
  → [.\core13.storyEngine.lore:305] when: causality.explain
  → [.\core13.storyEngine.lore:554] emit: causality.explain
  → [.\core13.storyEngine.lore:278] trigger: causality.explain
  → [.\core13.storyEngine.lore:305] when: causality.explain
  → [.\core13.storyEngine.lore:554] emit: causality.explain

🚨 Infinite loop detected on signal: 'gravity.override'
  → [.\core14.contextReflector.lore:184] trigger: gravity.override
  → [.\core14.contextReflector.lore:436] emit: gravity.override
  → [.\core14.contextReflector.lore:184] trigger: gravity.override
  → [.\core14.contextReflector.lore:436] emit: gravity.override

🚨 Infinite loop detected on signal: 'dream.inject'
  → [.\core24.symbolicCognition.lore:169] trigger: dream.inject
  → [.\core24.symbolicCognition.lore:376] emit: dream.inject
  → [.\core24.symbolicCognition.lore:169] trigger: dream.inject
  → [.\core24.symbolicCognition.lore:376] emit: dream.inject

🚨 Infinite loop detected on signal: 'agent.init'
  → [.\core8.agenticLayer.lore:11] trigger: agent.init
  → [.\core8.agenticLayer.lore:36] emit: agent.init
  → [.\core8.agenticLayer.lore:40] when: agent.init
  → [.\core8.agenticLayer.lore:11] trigger: agent.init
  → [.\core8.agenticLayer.lore:36] emit: agent.init
  → [.\core8.agenticLayer.lore:40] when: agent.init

🚨 Infinite loop detected on signal: 'capsule.export'
  → [.\core26.symbolicMeta.lore:44] trigger: capsule.export
  → [.\core26.symbolicMeta.lore:241] emit: capsule.export
  → [.\core26.symbolicMeta.lore:44] trigger: capsule.export
  → [.\core26.symbolicMeta.lore:241] emit: capsule.export

🚨 Infinite loop detected on signal: 'story.arc.define'
  → [.\core13.storyEngine.lore:186] trigger: story.arc.define
  → [.\core13.storyEngine.lore:262] when: story.arc.define
  → [.\core13.storyEngine.lore:534] emit: story.arc.define
  → [.\core13.storyEngine.lore:186] trigger: story.arc.define
  → [.\core13.storyEngine.lore:262] when: story.arc.define
  → [.\core13.storyEngine.lore:534] emit: story.arc.define

🚨 Infinite loop detected on signal: 'signalInspector.init'
  → [.\core16.signalInspector.lore:221] trigger: signalInspector.init
  → [.\core16.signalInspector.lore:416] emit: signalInspector.init
  → [.\core16.signalInspector.lore:221] trigger: signalInspector.init
  → [.\core16.signalInspector.lore:416] emit: signalInspector.init

🚨 Infinite loop detected on signal: 'memory.sync.quantum'
  → [.\core5.echoMesh.lore:155] trigger: memory.sync.quantum
  → [.\core5.echoMesh.lore:336] emit: memory.sync.quantum
  → [.\core5.echoMesh.lore:155] trigger: memory.sync.quantum
  → [.\core5.echoMesh.lore:336] emit: memory.sync.quantum

🚨 Infinite loop detected on signal: 'cap.token.create'
  → [.\core.kernel.lore:37] trigger: cap.token.create
  → [.\core.kernel.lore:562] emit: cap.token.create
  → [.\core.kernel.lore:37] trigger: cap.token.create
  → [.\core.kernel.lore:562] emit: cap.token.create

🚨 Infinite loop detected on signal: 'narrative.context.inject'
  → [.\core13.storyEngine.lore:344] trigger: narrative.context.inject
  → [.\core13.storyEngine.lore:366] when: narrative.context.inject
  → [.\core13.storyEngine.lore:489] emit: narrative.context.inject
  → [.\core13.storyEngine.lore:344] trigger: narrative.context.inject
  → [.\core13.storyEngine.lore:366] when: narrative.context.inject
  → [.\core13.storyEngine.lore:489] emit: narrative.context.inject

🚨 Infinite loop detected on signal: 'quantum.branch.check'
  → [.\core9.quantumBranch.lore:14] trigger: quantum.branch.check
  → [.\core9.quantumBranch.lore:68] emit: quantum.branch.check
  → [.\core9.quantumBranch.lore:14] trigger: quantum.branch.check
  → [.\core9.quantumBranch.lore:68] emit: quantum.branch.check

🚨 Infinite loop detected on signal: 'return.context'
  → [.\core0.boot.lore:471] trigger: return.context
  → [.\core0.boot.lore:499] emit: return.context
  → [.\core0.boot.lore:471] trigger: return.context
  → [.\core0.boot.lore:499] emit: return.context

🚨 Infinite loop detected on signal: 'trace.record'
  → [.\core16.signalInspector.lore:155] trigger: trace.record
  → [.\core16.signalInspector.lore:401] emit: trace.record
  → [.\core16.signalInspector.lore:155] trigger: trace.record
  → [.\core16.signalInspector.lore:401] emit: trace.record