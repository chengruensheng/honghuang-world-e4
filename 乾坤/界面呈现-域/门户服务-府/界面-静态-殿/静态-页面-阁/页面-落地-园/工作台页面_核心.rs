//! 页面-落地-园 - 工作台页面（真实数据驱动前端）
//!
//! 单文件前端：左侧边栏 + 顶部 7 Tab，数据全部来自 /api/* 真实接口。
//! 接口：/api/总览 /api/任务 /api/事件 /api/记忆 /api/仙官 /api/切面

/// 工作台页面 HTML（内嵌 CSS + JS，无外部依赖）
pub fn 工作台页面() -> String {
    r###"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>洪荒 · 智能体工坊 — 工作台</title>
<style>
  :root{
    --bg:#0b0e14;--bg2:#0f131c;--panel:#131824;--card:#181f2e;--card2:#1d2536;--card3:#222c40;
    --line:rgba(255,255,255,.07);--line2:rgba(255,255,255,.14);
    --tx:#e7eaf1;--tx2:#9aa4b8;--tx3:#626c80;
    --purple:#8b83e8;--gold:#e0b45c;--jade:#35c995;--amber:#efa33d;--red:#e85555;--blue:#4b91e0;
  }
  *{box-sizing:border-box;margin:0;padding:0}
  html,body{height:100%}
  body{background:var(--bg);color:var(--tx);font:13px/1.6 -apple-system,"PingFang SC","Microsoft YaHei","Segoe UI",sans-serif;display:flex;overflow:hidden;-webkit-font-smoothing:antialiased}
  ::-webkit-scrollbar{width:9px;height:9px}
  ::-webkit-scrollbar-thumb{background:#232b3d;border-radius:6px}
  ::-webkit-scrollbar-track{background:transparent}
  ::selection{background:rgba(139,131,232,.35)}
  .mono{font-variant-numeric:tabular-nums}
  .muted{color:var(--tx3)}
  .sb{width:224px;flex:none;background:var(--bg2);border-right:1px solid var(--line);display:flex;flex-direction:column;min-height:0}
  .sb-brand{display:flex;align-items:center;gap:10px;padding:15px 16px 13px;border-bottom:1px solid var(--line)}
  .mark{width:30px;height:30px;border-radius:8px;background:linear-gradient(135deg,#8b83e8,#e0b45c);display:flex;align-items:center;justify-content:center;font-size:14px;font-weight:700;color:#0b0e14}
  .sb-brand .t{font-size:14px;font-weight:600}
  .sb-brand .s{font-size:11px;color:var(--tx3)}
  .sb-body{flex:1;overflow-y:auto;padding:8px 8px 12px}
  .sb-sec{font-size:10.5px;color:var(--tx3);letter-spacing:.08em;padding:14px 10px 5px}
  .sess{padding:7px 10px;border-radius:8px;cursor:pointer}
  .sess:hover{background:var(--card)}
  .sess.on{background:var(--card2)}
  .sess .t{font-size:12.5px;color:var(--tx);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
  .sess .s{font-size:11px;color:var(--tx3)}
  .agent-mini{display:flex;align-items:center;gap:8px;padding:6px 10px;border-radius:8px;font-size:12.5px;color:var(--tx2)}
  .agent-mini .av{width:22px;height:22px;border-radius:6px;display:flex;align-items:center;justify-content:center;font-size:11px;font-weight:700;color:#0b0e14;flex:none}
  .agent-mini .st{margin-left:auto;font-size:11px;color:var(--tx3)}
  .dot{width:7px;height:7px;border-radius:50%;flex:none}
  .sb-foot{border-top:1px solid var(--line);padding:8px;font-size:12px;color:var(--tx3)}
  .main{flex:1;display:flex;flex-direction:column;min-width:0;min-height:0}
  .tbar{flex:none;background:var(--bg2);border-bottom:1px solid var(--line)}
  .tbar-top{height:50px;display:flex;align-items:center;gap:12px;padding:0 16px}
  .crumb{font-size:13px;color:var(--tx2)}
  .crumb b{color:var(--tx);font-weight:500}
  .spacer{flex:1}
  .pill{display:flex;align-items:center;gap:7px;border:1px solid var(--line);border-radius:9px;padding:5px 11px;font-size:12px;color:var(--tx2)}
  .pill b{color:var(--gold);font-weight:600;font-variant-numeric:tabular-nums}
  .pill .g{color:var(--jade)}
  .tbar-tabs{height:42px;display:flex;align-items:center;gap:2px;padding:0 16px}
  .tab{display:flex;align-items:center;gap:7px;padding:7px 15px;border-radius:8px;font-size:13px;color:var(--tx2);cursor:pointer;position:relative;user-select:none}
  .tab:hover{color:var(--tx);background:var(--card)}
  .tab.on{color:var(--tx);font-weight:600}
  .tab.on::after{content:'';position:absolute;left:14px;right:14px;bottom:-1px;height:2px;border-radius:2px;background:var(--purple)}
  .view{flex:1;min-height:0;display:none;overflow:hidden}
  .view.active{display:flex;flex-direction:column}
  .vwrap{flex:1;min-height:0;overflow-y:auto;padding:18px 20px 30px}
  .vhead{display:flex;align-items:flex-end;gap:14px;padding:14px 20px 0;flex:none}
  .vhead h1{font-size:17px;font-weight:600}
  .vhead .sub{font-size:12px;color:var(--tx3);margin-top:1px}
  .panel{background:var(--panel);border:1px solid var(--line);border-radius:12px;overflow:hidden}
  .panel-h{display:flex;align-items:center;gap:8px;padding:11px 14px;border-bottom:1px solid var(--line)}
  .panel-h .t{font-size:13px;font-weight:600}
  .panel-h .more{margin-left:auto;font-size:12px;color:var(--tx3)}
  .panel-b{padding:12px 14px}
  .grid{display:grid;gap:14px}
  .g4{grid-template-columns:repeat(4,1fr)}
  .g3{grid-template-columns:repeat(3,1fr)}
  .g2{grid-template-columns:3fr 2fr}
  .stat{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:14px 16px}
  .stat .l{font-size:12px;color:var(--tx2)}
  .stat .n{font-size:24px;font-weight:700;margin-top:6px;font-variant-numeric:tabular-nums}
  .stat .n small{font-size:12px;font-weight:500;color:var(--tx3)}
  .stat .d{font-size:11.5px;margin-top:4px;color:var(--tx3)}
  .bar{height:7px;border-radius:4px;background:var(--card2);overflow:hidden;flex:1}
  .bar i{display:block;height:100%;border-radius:4px}
  .kv{display:flex;flex-direction:column;gap:10px}
  .kv .r{display:flex;align-items:center;gap:10px;font-size:12px}
  .kv .r .k{width:130px;color:var(--tx2);flex:none}
  .kv .r .v{width:52px;text-align:right;font-variant-numeric:tabular-nums;flex:none}
  .badge{display:inline-flex;align-items:center;gap:5px;font-size:11px;padding:1px 8px;border-radius:20px;border:1px solid var(--line2);flex:none;white-space:nowrap}
  .badge.ok{color:var(--jade);border-color:rgba(53,201,149,.4)}
  .badge.run{color:var(--blue);border-color:rgba(75,145,224,.4)}
  .badge.wait{color:var(--amber);border-color:rgba(239,163,61,.4)}
  .badge.err{color:var(--red);border-color:rgba(232,85,85,.4)}
  .badge.pur{color:var(--purple);border-color:rgba(139,131,232,.4)}
  .badge.gold{color:var(--gold);border-color:rgba(224,180,92,.4)}
  .badge.plain{color:var(--tx2)}
  .table{width:100%;border-collapse:collapse;font-size:12.5px}
  .table th{text-align:left;font-size:11px;color:var(--tx3);font-weight:500;padding:8px 12px;border-bottom:1px solid var(--line);white-space:nowrap}
  .table td{padding:9px 12px;border-bottom:1px solid var(--line);color:var(--tx2);white-space:nowrap}
  .table tr:last-child td{border-bottom:none}
  .table tr:hover td{background:rgba(255,255,255,.015)}
  .table td b{color:var(--tx);font-weight:500}
  .ex{display:grid;grid-template-columns:260px 1fr;flex:1;min-height:0}
  .task-list{border-right:1px solid var(--line);overflow-y:auto;background:var(--bg2)}
  .task-item{padding:11px 14px;border-bottom:1px solid var(--line);cursor:pointer}
  .task-item:hover{background:var(--card)}
  .task-item.on{background:var(--card2);box-shadow:inset 3px 0 0 var(--purple)}
  .task-item .tt{font-size:12.5px;color:var(--tx)}
  .task-item .ts{font-size:11px;color:var(--tx3);margin-top:3px;display:flex;align-items:center;gap:6px}
  .traj{flex:1;min-height:0;overflow-y:auto;padding:16px 20px}
  .evt{border:1px solid var(--line);border-radius:10px;background:var(--panel);margin-bottom:8px;overflow:hidden}
  .evt .eh{display:flex;align-items:center;gap:9px;padding:9px 13px}
  .evt .t{font-size:11px;color:var(--tx3);font-variant-numeric:tabular-nums;flex:none}
  .evt .ti{font-size:12.5px;color:var(--tx);flex:1;min-width:0}
  .empty{text-align:center;color:var(--tx3);font-size:12px;padding:40px 10px}
  .filters{display:flex;gap:7px;flex-wrap:wrap;padding:12px 14px;border-bottom:1px solid var(--line);flex:none}
  .fchip{font-size:12px;color:var(--tx2);border:1px solid var(--line);border-radius:20px;padding:4px 13px;cursor:pointer}
  .fchip:hover{color:var(--tx);border-color:var(--line2)}
  .fchip.on{color:#0b0e14;background:var(--purple);border-color:var(--purple);font-weight:600}
  .agent-card{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:15px}
  .ac-h{display:flex;align-items:center;gap:11px}
  .ac-av{width:40px;height:40px;border-radius:10px;display:flex;align-items:center;justify-content:center;font-size:18px;font-weight:700;color:#0b0e14;flex:none}
  .ac-name{font-size:14px;font-weight:600}
  .ac-role{font-size:11.5px;color:var(--tx3)}
  .ac-body{margin-top:12px;display:flex;flex-direction:column;gap:7px;font-size:12px;color:var(--tx2)}
  .ac-body .r{display:flex;justify-content:space-between}
  .ac-body .r b{color:var(--tx);font-weight:500}
  .perm-tag{font-size:10.5px;padding:2px 8px;border-radius:5px;border:1px solid var(--line);color:var(--tx2);white-space:nowrap}
  .perm-tag.rw{color:var(--jade);border-color:rgba(53,201,149,.35)}
  .perm-tag.ro{color:var(--blue);border-color:rgba(75,145,224,.35)}
  .perm-tag.req{color:var(--amber);border-color:rgba(239,163,61,.35)}
  .tags{display:flex;flex-wrap:wrap;gap:5px;margin-top:3px}
  .asp-cat{font-size:12px;color:var(--tx3);margin:18px 0 9px;display:flex;align-items:center;gap:8px}
  .asp-cat::after{content:'';flex:1;height:1px;background:var(--line)}
  .asp-grid{display:grid;grid-template-columns:repeat(5,1fr);gap:10px}
  .asp-card{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:11px 12px}
  .asp-card .an{font-size:13px;font-weight:600;display:flex;align-items:center;gap:6px}
  .asp-card .ad{font-size:11px;color:var(--tx3);margin:6px 0 8px;min-height:32px;line-height:1.5}
  .asp-card textarea{width:100%;background:var(--card);border:1px solid var(--line);border-radius:6px;color:var(--tx);font-size:11px;line-height:1.45;padding:6px 8px;resize:vertical;min-height:40px;font-family:inherit}
  .asp-card textarea:focus{outline:none;border-color:var(--purple)}
  .canvas-wrap{flex:1;min-height:0;overflow:auto;padding:24px}
  .canvas{position:relative;width:640px;height:500px;margin:0 auto}
  .canvas svg{position:absolute;inset:0;width:100%;height:100%}
  .node{position:absolute;width:196px;background:var(--card);border:1px solid var(--line2);border-radius:12px;padding:11px 13px;z-index:2}
  .node .nh{display:flex;align-items:center;gap:8px;font-size:13px;font-weight:600}
  .node .nav{width:24px;height:24px;border-radius:6px;display:flex;align-items:center;justify-content:center;font-size:11px;font-weight:700;color:#0b0e14;flex:none}
  .node .nb{font-size:11px;color:var(--tx3);margin-top:7px;line-height:1.6}
  .sum-line{font-size:12px;color:var(--tx2);padding:2px 0 10px}
  .sum-line b{color:var(--tx)}
  @media(max-width:1200px){.g4{grid-template-columns:repeat(2,1fr)}.g2{grid-template-columns:1fr}.asp-grid{grid-template-columns:repeat(3,1fr)}}
</style>
</head>
<body>

<aside class="sb">
  <div class="sb-brand">
    <div class="mark">洪</div>
    <div><div class="t">智能体工坊</div><div class="s">多智能体协同工作台</div></div>
  </div>
  <div class="sb-body">
    <div class="sb-sec">最近会话</div>
    <div id="sideSess"></div>
    <div class="sb-sec">当值仙官</div>
    <div id="sideAgents"></div>
    <div class="sb-sec">资源</div>
    <div class="agent-mini">▤ 记忆条目 <span class="st" id="sideMem">—</span></div>
    <div class="agent-mini">◈ 上下文切面 <span class="st">20</span></div>
  </div>
  <div class="sb-foot">真实数据源 · 洪荒记忆库.sq3</div>
</aside>

<div class="main">
  <div class="tbar">
    <div class="tbar-top">
      <div class="crumb">乾坤 / 界面呈现-域 / 门户服务-府</div>
      <div class="spacer"></div>
      <div class="pill">◆ 记忆条目 <b id="pillMem">—</b></div>
      <div class="pill">✓ 已交付 <b class="g" id="pillDone">—</b></div>
      <div class="pill">↻ 打回 <b id="pillRej">—</b></div>
    </div>
    <div class="tbar-tabs" id="tabs">
      <div class="tab on" data-v="overview">◉ 总览</div>
      <div class="tab" data-v="exec">✦ 执行轨迹</div>
      <div class="tab" data-v="watch">◎ 事件流</div>
      <div class="tab" data-v="agents">☷ 仙官</div>
      <div class="tab" data-v="aspects">◈ 切面</div>
      <div class="tab" data-v="orch">⛓ 编排</div>
      <div class="tab" data-v="audit">⚖ 审计</div>
    </div>
  </div>

  <div class="view active" id="v-overview">
    <div class="vhead"><div><h1>总览工作台</h1><div class="sub">真实数据：任务账本 · 事件流 · 记忆条目</div></div></div>
    <div class="vwrap" id="ovBody"></div>
  </div>

  <div class="view" id="v-exec">
    <div class="vhead"><div><h1>执行轨迹</h1><div class="sub">按任务查看白箱轨迹（真实事件流）</div></div></div>
    <div class="ex">
      <div class="task-list" id="taskList"></div>
      <div class="traj" id="trajBody"></div>
    </div>
  </div>

  <div class="view" id="v-watch">
    <div class="vhead"><div><h1>事件流 · 运行观测</h1><div class="sub">92 条事件全量可回放</div></div></div>
    <div class="filters" id="feedFilters"></div>
    <div class="traj" id="feedBody"></div>
  </div>

  <div class="view" id="v-agents">
    <div class="vhead"><div><h1>仙官府</h1><div class="sub">智能体名册 · 修为等级与权限边界</div></div></div>
    <div class="vwrap" id="agBody"></div>
  </div>

  <div class="view" id="v-aspects">
    <div class="vhead"><div><h1>上下文切面</h1><div class="sub">二十切面 · 修仙化上下文体系</div></div></div>
    <div class="vwrap" id="aspBody"></div>
  </div>

  <div class="view" id="v-orch">
    <div class="vhead"><div><h1>编排工坊</h1><div class="sub">流水线：鸿钧 → 天机/文曲 → 天道 → 发布</div></div></div>
    <div class="canvas-wrap">
      <div class="canvas">
        <svg viewBox="0 0 640 500">
          <g stroke="rgba(255,255,255,.16)" stroke-width="1.6" fill="none">
            <path d="M320,96 C320,128 220,140 220,168"/>
            <path d="M320,96 C320,128 420,140 420,168"/>
            <path d="M220,292 C220,320 300,330 315,350"/>
            <path d="M420,292 C420,320 340,330 325,350"/>
            <path d="M320,404 C320,424 320,434 320,448"/>
          </g>
        </svg>
        <div class="node" style="left:222px;top:32px"><div class="nh"><span class="nav" style="background:#e0b45c">鸿</span>鸿钧 · 总调度</div><div class="nb">道祖 · 目标对齐 · 任务拆解</div></div>
        <div class="node" style="left:24px;top:184px"><div class="nh"><span class="nav" style="background:#35c995">机</span>天机 · 情报推演</div><div class="nb">圣人 · 搜索 / 抓取 / 对比</div></div>
        <div class="node" style="left:420px;top:184px"><div class="nh"><span class="nav" style="background:#8b83e8">曲</span>文曲 · 著述</div><div class="nb">大罗 · 知识库 · 写作</div></div>
        <div class="node" style="left:222px;top:364px"><div class="nh"><span class="nav" style="background:#efa33d">道</span>天道 · 审批</div><div class="nb">准圣 · 业障红线把关</div></div>
        <div class="node" style="left:222px;top:448px"><div class="nh"><span class="nav" style="background:#e85555">业</span>发布 · 业障红线</div><div class="nb">外发 / 支付 / 删除须授权</div></div>
      </div>
    </div>
  </div>

  <div class="view" id="v-audit">
    <div class="vhead"><div><h1>业障审计</h1><div class="sub">红线操作留痕 · 打回与终裁统计</div></div></div>
    <div class="vwrap" id="auBody"></div>
  </div>
</div>

<script>
var 数据 = { 总览:null, 任务:[], 事件:[], 记忆:[], 仙官:[], 切面:[] };
function 取(路径){ return fetch(路径).then(function(r){ return r.json(); }); }
function 时间戳转文本(秒){
  if(!秒) return '—';
  var d = new Date(Number(秒) * 1000);
  var pad = function(n){ return (n < 10 ? '0' : '') + n; };
  return d.getFullYear()+'-'+pad(d.getMonth()+1)+'-'+pad(d.getDate())+' '+pad(d.getHours())+':'+pad(d.getMinutes());
}
var 事件颜色 = {
  '终裁通过交付':['ok','✓ 终裁通过'],
  '打回重投':['run','↻ 打回重投'],
  '打回达上限':['err','✕ 打回达上限'],
  '终裁打回':['wait','⚠ 终裁打回']
};
var 总纲颜色 = {
  '内部':'#8b83e8','外在':'#4b91e0','执行':'#35c995','目标':'#e0b45c','经历':'#efa33d','规则':'#e85555'
};
function 事件徽章(类型){
  var c = 事件颜色[类型] || ['plain',类型];
  return '<span class="badge '+c[0]+'">'+c[1]+'</span>';
}

/* ---------- 顶部 Tab ---------- */
var 当前页 = 'overview';
document.getElementById('tabs').addEventListener('click', function(e){
  var t = e.target.closest('.tab'); if(!t) return;
  document.querySelectorAll('.tab').forEach(function(x){ x.classList.remove('on'); });
  t.classList.add('on');
  document.querySelectorAll('.view').forEach(function(x){ x.classList.remove('active'); });
  document.getElementById('v-'+t.dataset.v).classList.add('active');
  当前页 = t.dataset.v;
  if(当前页==='exec') 渲染执行();
  if(当前页==='watch') 渲染事件流();
  if(当前页==='audit') 渲染审计();
});

/* ---------- 概览 ---------- */
function 渲染概览(){
  var g = 数据.总览;
  var 事件分布 = {};
  数据.事件.forEach(function(e){ 事件分布[e.事件类型] = (事件分布[e.事件类型]||0)+1; });
  var 分布色 = { '终裁通过交付':'#35c995','打回重投':'#4b91e0','打回达上限':'#e85555','终裁打回':'#efa33d' };
  var 分布条 = Object.keys(事件颜色).map(function(k){
    var n = 事件分布[k]||0, pct = g.事件总数 ? Math.round(n/g.事件总数*100) : 0;
    return '<div class="r"><span class="k">'+事件颜色[k][1]+'</span><div class="bar"><i style="width:'+pct+'%;background:'+分布色[k]+'"></i></div><span class="v">'+n+'</span></div>';
  }).join('');
  var 任务行 = 数据.任务.slice(0,8).map(function(t){
    return '<tr><td><b>'+t.任务标识+'</b></td><td>'+(t.已交付?'<span class="badge ok">✓ 已交付</span>':'<span class="badge run">进行中</span>')+'</td><td>'+(t.已归档?'<span class="badge plain">已归档</span>':'<span class="badge plain">未归档</span>')+'</td><td class="mono">'+时间戳转文本(t.更新时间)+'</td></tr>';
  }).join('');
  document.getElementById('ovBody').innerHTML =
    '<div class="grid g4" style="margin-bottom:14px">'+
      '<div class="stat"><div class="l">任务总数</div><div class="n">'+g.任务总数+'<small>个</small></div><div class="d">已交付 '+g.已交付+' · 已归档 '+g.已归档+'</div></div>'+
      '<div class="stat"><div class="l">事件总数</div><div class="n">'+g.事件总数+'<small>条</small></div><div class="d">终裁通过 '+g.终裁通过+' 次</div></div>'+
      '<div class="stat"><div class="l">累计打回</div><div class="n" style="color:var(--red)">'+g.打回数+'<small>次</small></div><div class="d">重投 + 达上限 + 终裁打回</div></div>'+
      '<div class="stat"><div class="l">记忆条目</div><div class="n" style="color:var(--purple)">'+g.记忆总数+'<small>条</small></div><div class="d">36 格位 · 3 档投影</div></div>'+
    '</div>'+
    '<div class="grid g2" style="margin-bottom:14px">'+
      '<div class="panel"><div class="panel-h"><span class="t">事件类型分布</span><span class="more">真实统计</span></div><div class="panel-b"><div class="kv">'+分布条+'</div></div></div>'+
      '<div class="panel"><div class="panel-h"><span class="t">数据源概览</span><span class="more">洪荒记忆库.sq3</span></div><div class="panel-b" style="font-size:12px;color:var(--tx2);line-height:1.8">'+
        '<div>任务账本 · 18 行</div><div>事件流 · '+g.事件总数+' 行</div><div>记忆条目 · '+g.记忆总数+' 行</div><div class="muted" style="margin-top:4px">已交付率 '+(g.任务总数?Math.round(g.已交付/g.任务总数*100):0)+'% · 终裁通过率 '+(g.事件总数?Math.round(g.终裁通过/g.事件总数*100):0)+'%</div>'+
      '</div></div>'+
    '</div>'+
    '<div class="panel"><div class="panel-h"><span class="t">任务账本</span><span class="more">按更新时间倒序 · 前 8 条</span></div><div class="panel-b" style="padding:0"><table class="table"><tr><th>任务</th><th>交付</th><th>归档</th><th>更新时间</th></tr>'+任务行+'</table></div></div>';
}

/* ---------- 侧栏 ---------- */
function 渲染侧栏(){
  document.getElementById('sideSess').innerHTML = 数据.任务.slice(0,5).map(function(t,i){
    return '<div class="sess'+(i===0?' on':'')+'"><div class="t">'+t.任务标识+'</div><div class="s">'+(t.已交付?'已交付':'进行中')+' · '+时间戳转文本(t.更新时间).slice(0,10)+'</div></div>';
  }).join('');
  document.getElementById('sideAgents').innerHTML = 数据.仙官.map(function(a){
    return '<div class="agent-mini"><span class="av" style="background:'+a.颜色+'">'+a.字+'</span>'+a.名字+' · '+a.职责+'<span class="st"><span class="dot" style="background:var(--jade)"></span></span></div>';
  }).join('');
  document.getElementById('sideMem').textContent = 数据.记忆.length;
  document.getElementById('pillMem').textContent = 数据.总览.记忆总数;
  document.getElementById('pillDone').textContent = 数据.总览.已交付;
  document.getElementById('pillRej').textContent = 数据.总览.打回数;
}

/* ---------- 执行轨迹 ---------- */
var 选中任务 = null;
function 渲染执行(){
  var 列表 = document.getElementById('taskList');
  列表.innerHTML = 数据.任务.map(function(t){
    var 该任务事件 = 数据.事件.filter(function(e){ return e.内容 === t.任务标识; });
    var 通过 = 该任务事件.filter(function(e){ return e.事件类型==='终裁通过交付'; }).length;
    return '<div class="task-item'+(选中任务===t.任务标识?' on':'')+'" data-t="'+t.任务标识+'"><div class="tt">'+t.任务标识+'</div>'+
      '<div class="ts"><span class="badge '+(t.已交付?'ok':'run')+'">'+(t.已交付?'✓ 交付':'进行中')+'</span> 事件 '+该任务事件.length+' · 通过 '+通过+'</div></div>';
  }).join('');
  列表.querySelectorAll('.task-item').forEach(function(el){
    el.addEventListener('click', function(){ 选中任务 = el.dataset.t; 渲染执行(); });
  });
  if(!选中任务 && 数据.任务.length) 选中任务 = 数据.任务[0].任务标识;
  var 事件们 = 数据.事件.filter(function(e){ return e.内容 === 选中任务; }).sort(function(a,b){ return a.序号 - b.序号; });
  var 轨迹 = document.getElementById('trajBody');
  if(!事件们.length){
    轨迹.innerHTML = '<div class="empty">该任务暂无事件留痕</div>'; return;
  }
  var 通过 = 事件们.filter(function(e){ return e.事件类型==='终裁通过交付'; }).length;
  var 打回 = 事件们.length - 通过;
  轨迹.innerHTML = '<div class="sum-line">任务 <b>'+选中任务+'</b> · 事件 <b>'+事件们.length+'</b> 条 · 终裁通过 <b style="color:var(--jade)">'+通过+'</b> 次 · 打回 <b style="color:var(--red)">'+打回+'</b> 次</div>'+
    事件们.map(function(e){
       return '<div class="evt"><div class="eh"><span class="t">'+时间戳转文本(e.时间戳)+'</span>'+事件徽章(e.事件类型)+'<span class="ti">#'+e.序号+' · '+e.内容+'</span></div></div>';
     }).join('');
}

/* ---------- 事件流 ---------- */
var 过滤类型 = '全部';
function 渲染事件流(){
  var 容器 = document.getElementById('feedFilters');
  var 类型们 = ['全部'].concat(Object.keys(事件颜色));
  容器.innerHTML = 类型们.map(function(k){
    return '<div class="fchip'+(过滤类型===k?' on':'')+'" data-k="'+k+'">'+k+'</div>';
  }).join('');
  容器.querySelectorAll('.fchip').forEach(function(c){
    c.addEventListener('click', function(){ 过滤类型 = c.dataset.k; 渲染事件流(); });
  });
  var 事件们 = 数据.事件.slice().sort(function(a,b){ return b.序号 - a.序号; })
    .filter(function(e){ return 过滤类型==='全部' || e.事件类型===过滤类型; });
  document.getElementById('feedBody').innerHTML = 事件们.map(function(e){
    return '<div class="evt"><div class="eh"><span class="t">'+时间戳转文本(e.时间戳)+'</span>'+事件徽章(e.事件类型)+'<span class="ti">'+e.内容+'</span><span class="muted" style="font-size:11px">#'+e.序号+'</span></div></div>';
  }).join('') || '<div class="empty">无匹配事件</div>';
}

/* ---------- 仙官 ---------- */
function 渲染仙官(){
  document.getElementById('agBody').innerHTML =
    '<div class="grid g4" style="margin-bottom:14px">'+数据.仙官.map(function(a){
      return '<div class="agent-card"><div class="ac-h"><div class="ac-av" style="background:'+a.颜色+'">'+a.字+'</div><div><div class="ac-name">'+a.名字+'</div><div class="ac-role">'+a.职责+' · '+a.等级+'</div></div></div>'+
        '<div class="ac-body"><div class="r"><span>道力配额</span><b>'+a.道力配额+'</b></div><div class="r"><span>绑定</span><b>'+a.绑定+'</b></div>'+
        '<div class="tags">'+a.权限.map(function(p){ return '<span class="perm-tag">'+p+'</span>'; }).join('')+'</div></div></div>';
    }).join('')+'</div>'+
    '<div class="panel"><div class="panel-h"><span class="t">权限矩阵 · CQRS 唯一写入者</span><span class="more">按角色与作用域授权</span></div><div class="panel-b" style="padding:0"><table class="table">'+
      '<tr><th>仙官</th><th>本心</th><th>道心</th><th>业障</th><th>天机</th><th>外发发布</th><th>支付</th><th>删除</th></tr>'+
      '<tr><td><b>鸿钧</b></td><td><span class="perm-tag rw">读写</span></td><td><span class="perm-tag rw">读写</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag rw">读写</span></td><td><span class="perm-tag req">审批</span></td><td><span class="perm-tag req">审批</span></td><td><span class="perm-tag req">审批</span></td></tr>'+
      '<tr><td><b>天机</b></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag">无</span></td><td><span class="perm-tag">无</span></td><td><span class="perm-tag rw">读写</span></td><td><span class="perm-tag">无</span></td><td><span class="perm-tag">无</span></td><td><span class="perm-tag">无</span></td></tr>'+
      '<tr><td><b>文曲</b></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag req">需审批</span></td><td><span class="perm-tag">无</span></td><td><span class="perm-tag req">需审批</span></td></tr>'+
      '<tr><td><b>天道</b></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag rw">读写</span></td><td><span class="perm-tag ro">只读</span></td><td><span class="perm-tag rw">拦截</span></td><td><span class="perm-tag rw">拦截</span></td><td><span class="perm-tag rw">拦截</span></td></tr>'+
    '</table></div></div>';
}

/* ---------- 切面 ---------- */
function 渲染切面(){
  document.getElementById('aspBody').innerHTML = 数据.切面.map(function(g){
    return '<div class="asp-cat"><span style="color:'+g.颜色+'">▸</span>'+g.类别+'（'+g.条目.length+'）</div>'+
      '<div class="asp-grid">'+g.条目.map(function(it){
        return '<div class="asp-card"><div class="an"><span style="color:'+g.颜色+'">'+it.符号+'</span>'+it.名+'</div><div class="ad">'+it.释义+'</div><textarea placeholder="编辑此切面…">'+it.释义+'</textarea></div>';
      }).join('')+'</div>';
  }).join('');
}

/* ---------- 审计 ---------- */
function 渲染审计(){
  var 达上限 = {}; var 终裁打回 = {};
  数据.事件.forEach(function(e){
    if(e.事件类型==='打回达上限') 达上限[e.内容] = (达上限[e.内容]||0)+1;
    if(e.事件类型==='终裁打回') 终裁打回[e.内容] = (终裁打回[e.内容]||0)+1;
  });
  var 达上限行 = Object.keys(达上限).map(function(k){
    return '<tr><td><b>'+k+'</b></td><td><span class="badge err">✕ 打回达上限</span></td><td class="mono">'+达上限[k]+' 次</td></tr>';
  }).join('') || '<tr><td colspan="3" class="muted">无</td></tr>';
  var 打回行 = Object.keys(终裁打回).map(function(k){
    return '<tr><td><b>'+k+'</b></td><td><span class="badge wait">⚠ 终裁打回</span></td><td class="mono">'+终裁打回[k]+' 次</td></tr>';
  }).join('') || '<tr><td colspan="3" class="muted">无</td></tr>';
  var g = 数据.总览;
  document.getElementById('auBody').innerHTML =
    '<div class="grid g3" style="margin-bottom:14px">'+
      '<div class="stat"><div class="l">累计打回</div><div class="n" style="color:var(--red)">'+g.打回数+'<small>次</small></div><div class="d">重投 '+数据.事件.filter(function(e){return e.事件类型==='打回重投';}).length+' · 达上限 '+Object.keys(达上限).length+' 任务 · 终裁打回 '+Object.keys(终裁打回).length+' 任务</div></div>'+
      '<div class="stat"><div class="l">终裁通过</div><div class="n" style="color:var(--jade)">'+g.终裁通过+'<small>次</small></div><div class="d">通过率 '+(g.事件总数?Math.round(g.终裁通过/g.事件总数*100):0)+'%</div></div>'+
      '<div class="stat"><div class="l">任务交付</div><div class="n">'+g.已交付+'<small>/'+g.任务总数+'</small></div><div class="d">交付率 '+(g.任务总数?Math.round(g.已交付/g.任务总数*100):0)+'%</div></div>'+
    '</div>'+
    '<div class="grid g2">'+
      '<div class="panel"><div class="panel-h"><span class="t">打回达上限（放弃）</span></div><div class="panel-b" style="padding:0"><table class="table"><tr><th>任务</th><th>结果</th><th>次数</th></tr>'+达上限行+'</table></div></div>'+
      '<div class="panel"><div class="panel-h"><span class="t">终裁打回</span></div><div class="panel-b" style="padding:0"><table class="table"><tr><th>任务</th><th>结果</th><th>次数</th></tr>'+打回行+'</table></div></div>'+
    '</div>';
}

/* ---------- 启动 ---------- */
Promise.all([
  取('/api/总览'), 取('/api/任务'), 取('/api/事件'), 取('/api/记忆'), 取('/api/仙官'), 取('/api/切面')
]).then(function(res){
  数据.总览 = res[0];
  数据.任务 = res[1].任务 || [];
  数据.事件 = res[2].事件 || [];
  数据.记忆 = res[3].记忆 || [];
  数据.仙官 = res[4].仙官 || [];
  数据.切面 = res[5].切面 || [];
  渲染侧栏();
  渲染概览();
  渲染仙官();
  渲染切面();
  渲染事件流();
}).catch(function(错){
  document.getElementById('ovBody').innerHTML = '<div class="empty">数据加载失败：'+错+'</div>';
});
</script>
</body>
</html>"###.to_string()
}
