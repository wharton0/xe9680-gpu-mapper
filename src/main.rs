#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

// GPU 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GpuInfo {
    minor_number: String,
    bus_id: String,
    module_id: String,
    slot: String,
    psb: String,
}

fn main() -> wry::Result<()> {
    // 创建事件循环
    let event_loop = EventLoop::new();
    
    // 创建窗口
    let window = WindowBuilder::new()
        .with_title("Dell XE9680 GPU 映射查询程序 - Wharton Wang v1.7")
        .with_inner_size(wry::application::dpi::LogicalSize::new(1400.0, 900.0))
        .build(&event_loop)?;

    // GPU 数据
    let gpu_data = vec![
        GpuInfo {
            minor_number: "0".to_string(),
            bus_id: "19".to_string(),
            module_id: "gpu2".to_string(),
            slot: "s28".to_string(),
            psb: "psb1".to_string(),
        },
        GpuInfo {
            minor_number: "1".to_string(),
            bus_id: "3B".to_string(),
            module_id: "gpu4".to_string(),
            slot: "s24".to_string(),
            psb: "psb1".to_string(),
        },
        GpuInfo {
            minor_number: "2".to_string(),
            bus_id: "4C".to_string(),
            module_id: "gpu1".to_string(),
            slot: "s23".to_string(),
            psb: "psb2".to_string(),
        },
        GpuInfo {
            minor_number: "3".to_string(),
            bus_id: "5D".to_string(),
            module_id: "gpu3".to_string(),
            slot: "s27".to_string(),
            psb: "psb2".to_string(),
        },
        GpuInfo {
            minor_number: "4".to_string(),
            bus_id: "9B".to_string(),
            module_id: "gpu7".to_string(),
            slot: "s25".to_string(),
            psb: "psb4".to_string(),
        },
        GpuInfo {
            minor_number: "5".to_string(),
            bus_id: "BB".to_string(),
            module_id: "gpu5".to_string(),
            slot: "s21".to_string(),
            psb: "psb4".to_string(),
        },
        GpuInfo {
            minor_number: "6".to_string(),
            bus_id: "CB".to_string(),
            module_id: "gpu6".to_string(),
            slot: "s26".to_string(),
            psb: "psb3".to_string(),
        },
        GpuInfo {
            minor_number: "7".to_string(),
            bus_id: "DB".to_string(),
            module_id: "gpu8".to_string(),
            slot: "s22".to_string(),
            psb: "psb3".to_string(),
        },
    ];

    // 将数据嵌入到 HTML 中
    let gpu_data_json = serde_json::to_string(&gpu_data).unwrap();
    
    // 完整的 HTML 内容（内联所有资源）
    let html = format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>XE9680 GPU 映射查询程序</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background-color: #1a1c23;
            color: #e2e8f0;
        }}
        .fade-in {{
            animation: fadeIn 0.3s ease-in-out;
        }}
        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(10px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        .gpu-module {{
            background-color: #0f0f0f;
            background-image: repeating-linear-gradient(
                0deg,
                #0f0f0f,
                #0f0f0f 4px,
                #2a2a2a 4px,
                #2a2a2a 5px
            );
            border: 1px solid #333;
            box-shadow: 0 4px 6px rgba(0,0,0,0.5);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            position: relative;
        }}

        .gpu-plate {{
            background: linear-gradient(180deg, #e3dfce 0%, #f7f4e6 50%, #cfcaba 100%);
            border-radius: 4px;
            box-shadow: 0 1px 2px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.8);
            color: #333;
            transition: all 0.3s ease;
        }}

        .screw {{
            width: 6px;
            height: 6px;
            background: radial-gradient(circle at 30% 30%, #fff, #555);
            border-radius: 50%;
            position: absolute;
            box-shadow: 0 1px 1px rgba(0,0,0,0.4);
        }}
        .screw-tl {{ top: 6px; left: 6px; }}
        .screw-tr {{ top: 6px; right: 6px; }}
        .screw-bl {{ bottom: 6px; left: 6px; }}
        .screw-br {{ bottom: 6px; right: 6px; }}

        .gpu-module.active {{
            transform: scale(1.05);
            border-color: #ef4444;
            box-shadow: 0 0 20px rgba(239, 68, 68, 0.6);
            z-index: 10;
        }}
        .gpu-module.active .gpu-plate {{
            background: linear-gradient(180deg, #fecaca 0%, #fee2e2 50%, #f87171 100%);
        }}
        .gpu-module.active .bus-id {{
            color: #991b1b;
            text-shadow: 0 1px 0 rgba(255,255,255,0.4);
        }}

        .writing-vertical-rl {{
            writing-mode: vertical-rl;
            text-orientation: mixed;
        }}

        .rust-badge {{
            background: linear-gradient(135deg, #ff6b35 0%, #f7931e 100%);
            color: white;
            padding: 4px 12px;
            border-radius: 6px;
            font-size: 12px;
            font-weight: bold;
            box-shadow: 0 2px 4px rgba(0,0,0,0.3);
        }}
    </style>
</head>
<body class="min-h-screen flex flex-col items-center py-10 px-4">

    <header class="text-center mb-6 w-full max-w-6xl">
        <div class="flex items-center justify-center gap-3 mb-2">
            <h1 class="text-4xl font-extrabold text-white tracking-tight">Dell XE9680 GPU 映射查询程序</h1>
            <span class="rust-badge">Desktop App</span>
        </div>
        <p class="text-sm text-gray-400">Wharton Wang v1.7</p>
        <p class="text-gray-500 mt-4 w-full text-sm whitespace-nowrap overflow-hidden text-ellipsis">
            查询操作系统 ID 与物理 GPU 位置的对应关系。输入任意字段值（如 "0", "gpu1", "3B"）即可高亮显示物理模组。
        </p>
    </header>

    <div class="w-full max-w-md mb-6">
        <div class="relative group">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg class="h-5 w-5 text-gray-400 group-focus-within:text-green-400 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                </svg>
            </div>
            <input 
                type="text" 
                id="searchInput"
                class="block w-full pl-10 pr-4 py-3 border-2 border-gray-600 rounded-lg leading-5 bg-gray-800 text-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:border-green-500 focus:ring-1 focus:ring-green-500 sm:text-lg transition duration-150 ease-in-out shadow-lg"
                placeholder="输入 Bus ID (如 3B), GPU ID..."
                autofocus
            >
            <div class="absolute inset-y-0 right-0 pr-3 flex items-center">
                <span id="clearBtn" class="cursor-pointer text-gray-400 hover:text-white hidden">
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                </span>
            </div>
        </div>
    </div>

    <div class="flex flex-col lg:flex-row w-full max-w-7xl gap-8 items-start justify-center">
        
        <div class="w-full max-w-lg flex-shrink-0">
            <h3 class="text-gray-400 font-bold mb-6 text-center text-lg tracking-wide uppercase">GPU 物理位置示意图</h3>
            
            <div class="flex items-center justify-center gap-6">
                
                <div class="flex flex-col items-center justify-center h-full text-gray-500 font-bold text-lg">
                    <div class="border-r-2 border-gray-700 pr-3 h-80 flex items-center writing-vertical-rl">
                        后 (Rear)
                    </div>
                </div>

                <div class="bg-gray-900 p-6 rounded-2xl shadow-2xl border border-gray-800 flex-1 relative">
                    <div class="absolute top-0 left-1/2 transform -translate-x-1/2 w-1/3 h-1 bg-gray-700 rounded-b"></div>

                    <div id="gpuGrid" class="grid grid-cols-2 gap-x-4 gap-y-3 text-center">
                    </div>
                </div>

                <div class="flex flex-col items-center justify-center h-full text-gray-500 font-bold text-lg">
                    <div class="border-l-2 border-gray-700 pl-3 h-80 flex items-center writing-vertical-rl">
                        前 (Front)
                    </div>
                </div>

            </div>
            
            <p class="text-xs text-gray-500 mt-4 text-center">* 红色高亮表示当前查询匹配的 GPU 位置</p>
        </div>

        <div class="flex-1 w-full">
            <h3 class="text-gray-400 font-bold mb-4 text-center md:text-left text-lg tracking-wide uppercase">查询结果详情</h3>
            <div id="resultsContainer" class="grid grid-cols-1 gap-4">
                <div class="bg-gray-800 rounded-xl border border-gray-700 p-10 text-center text-gray-500 italic shadow-inner text-xl">
                    请输入查询条件以显示详细信息...
                </div>
            </div>

            <div class="mt-8 text-base text-gray-400 bg-gray-800 p-6 rounded-xl shadow-md border border-gray-700">
                <h3 class="font-bold text-gray-200 mb-3 border-b border-gray-600 pb-2 text-lg">字段说明:</h3>
                <ul class="space-y-2 list-disc list-inside text-sm">
                    <li><span class="font-semibold text-gray-300">Minor Number:</span> 操作系统内的设备ID</li>
                    <li><span class="font-semibold text-gray-300">Module ID:</span> GPU模块的物理ID</li>
                    <li><span class="font-semibold text-gray-300">Slot:</span> GPU所在的物理槽位</li>
                    <li><span class="font-semibold text-gray-300">Bus ID:</span> GPU的PCIe总线地址 (Hex)</li>
                    <li><span class="font-semibold text-gray-300">PSB:</span> 所属的PSB板</li>
                </ul>
            </div>
        </div>

    </div>

    <script>
        const gpuData = {gpu_data_json};
        
        const gpuLayout = ['gpu2', 'gpu4', 'gpu3', 'gpu1', 'gpu6', 'gpu8', 'gpu7', 'gpu5'];

        const fieldLabels = {{
            minor_number: "Minor Number",
            bus_id: "Bus ID",
            module_id: "Module ID",
            slot: "Slot",
            psb: "PSB"
        }};

        const searchInput = document.getElementById('searchInput');
        const resultsContainer = document.getElementById('resultsContainer');
        const clearBtn = document.getElementById('clearBtn');
        const gpuGrid = document.getElementById('gpuGrid');

        function renderGpuGrid() {{
            gpuGrid.innerHTML = '';
            
            gpuLayout.forEach(moduleId => {{
                const gpu = gpuData.find(g => g.module_id === moduleId);
                if (!gpu) return;

                const gpuDiv = document.createElement('div');
                gpuDiv.className = 'gpu-module rounded-lg p-2 flex flex-col items-center justify-center h-24';
                gpuDiv.dataset.moduleId = moduleId;
                
                const gpuNumber = moduleId.replace('gpu', '');
                const slotNumber = gpu.slot.replace('s', '');
                
                gpuDiv.innerHTML = `
                    <div class="gpu-plate w-full h-full rounded flex flex-col items-center justify-center relative">
                        <div class="screw screw-tl"></div><div class="screw screw-tr"></div><div class="screw screw-bl"></div><div class="screw screw-br"></div>
                        <div class="flex items-center gap-1 mb-0.5 opacity-60"><svg class="w-3 h-3" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12s5.37 12 12 12 12-5.37 12-12S18.63 0 12 0zm0 2c2.39 0 4.6.83 6.36 2.21l-2.2 3.81c-1.14-.77-2.52-1.22-4.16-1.22-3.31 0-6 2.69-6 6s2.69 6 6 6c1.64 0 3.02-.45 4.16-1.22l2.2 3.81C16.6 23.17 14.39 24 12 24 5.37 24 0 18.63 0 12S5.37 0 12 0z"/></svg><span class="text-[10px] font-bold tracking-widest text-gray-600">NVIDIA</span></div>
                        <span class="bus-id text-2xl text-gray-800 font-black tracking-wider font-mono">${{gpu.bus_id}}</span>
                        <div class="flex flex-col leading-tight mt-0.5"><span class="text-xs font-bold text-gray-700">GPU ${{gpuNumber}}</span><span class="text-[9px] text-gray-500">Slot ${{slotNumber}}</span></div>
                    </div>
                `;
                
                gpuGrid.appendChild(gpuDiv);
            }});
        }}

        function performSearch(query) {{
            resultsContainer.innerHTML = '';
            
            const visualBoxes = document.querySelectorAll('.gpu-module');
            visualBoxes.forEach(box => box.classList.remove('active'));

            if (!query) {{
                clearBtn.classList.add('hidden');
                resultsContainer.innerHTML = '<div class="bg-gray-800 rounded-xl border border-gray-700 p-10 text-center text-gray-500 italic shadow-inner text-xl">请输入查询条件以显示详细信息...</div>';
                return;
            }}
            clearBtn.classList.remove('hidden');

            const normalizedQuery = query.trim().toLowerCase();

            const matches = gpuData.filter(item => {{
                return Object.values(item).some(val => val.toLowerCase() === normalizedQuery);
            }});

            if (matches.length === 0) {{
                resultsContainer.innerHTML = `
                    <div class="col-span-full text-center py-10 text-gray-400 bg-gray-800 rounded-lg shadow border border-gray-700 text-xl">
                        <p>未找到匹配项: "${{query}}"</p>
                    </div>
                `;
                return;
            }}

            matches.forEach(item => {{
                const targetBox = document.querySelector(`.gpu-module[data-module-id="${{item.module_id}}"]`);
                if (targetBox) {{
                    targetBox.classList.add('active');
                }}

                const card = document.createElement('div');
                card.className = 'bg-gray-800 rounded-xl shadow-lg border-l-8 border-green-500 p-8 fade-in hover:shadow-xl transition-shadow border border-gray-700';
                
                let htmlContent = '<div class="grid grid-cols-2 gap-y-4 text-xl">'; 
                
                for (const [key, value] of Object.entries(item)) {{
                    const isMatched = value.toLowerCase() === normalizedQuery;
                    const valueClass = isMatched 
                        ? 'font-extrabold text-green-400 bg-green-900 bg-opacity-30 px-3 py-0.5 rounded inline-block text-2xl' 
                        : 'text-gray-200 font-semibold';

                    htmlContent += `
                        <div class="text-gray-400 flex items-center font-medium">${{fieldLabels[key]}}:</div>
                        <div class="text-right sm:text-left"><span class="${{valueClass}}">${{value}}</span></div>
                    `;
                }}
                
                htmlContent += '</div>';
                card.innerHTML = htmlContent;
                resultsContainer.appendChild(card);
            }});
        }}

        searchInput.addEventListener('input', (e) => {{
            performSearch(e.target.value);
        }});

        clearBtn.addEventListener('click', () => {{
            searchInput.value = '';
            performSearch('');
            searchInput.focus();
        }});

        renderGpuGrid();
    </script>
</body>
</html>
"#, gpu_data_json = gpu_data_json);

    // 创建 WebView
    let _webview = WebViewBuilder::new(window)?
        .with_html(&html)?
        .build()?;

    // 运行事件循环
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Dell XE9680 GPU 映射查询程序已启动"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
