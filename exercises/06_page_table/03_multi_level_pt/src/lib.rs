//! # SV39 三级页表
//!
//! 本练习模拟 RISC-V SV39 三级页表的构造和地址翻译。
//! 注意，实际上的三级页表实现并非如本练习中使用 HashMap 模拟，本练习仅作为模拟帮助学习。
//! 你需要实现页表的创建、映射和地址翻译（页表遍历）。
//!
//! ## 知识点
//! - SV39：39 位虚拟地址，三级页表
//! - VPN 拆分：VPN[2] (9bit) | VPN[1] (9bit) | VPN[0] (9bit)
//! - 页表遍历（page table walk）逐级查找
//! - 大页（2MB superpage）映射
//!
//! ## SV39 虚拟地址布局
//! ```text
//! 38        30 29       21 20       12 11        0
//! ┌──────────┬───────────┬───────────┬───────────┐
//! │ VPN[2]   │  VPN[1]   │  VPN[0]   │  offset   │
//! │  9 bits  │  9 bits   │  9 bits   │  12 bits  │
//! └──────────┴───────────┴───────────┴───────────┘
//! ```

use std::collections::HashMap;

/// 页大小 4KB
pub const PAGE_SIZE: usize = 4096;
/// 每级页表有 512 个条目 (2^9)
pub const PT_ENTRIES: usize = 512;

/// PTE 标志位
pub const PTE_V: u64 = 1 << 0;
pub const PTE_R: u64 = 1 << 1;
pub const PTE_W: u64 = 1 << 2;
pub const PTE_X: u64 = 1 << 3;

/// PPN 在 PTE 中的偏移
const PPN_SHIFT: u32 = 10;

/// 页表节点：一个包含 512 个条目的数组
#[derive(Clone)]
pub struct PageTableNode {
    pub entries: [u64; PT_ENTRIES],
}

impl PageTableNode {
    pub fn new() -> Self {
        Self {
            entries: [0; PT_ENTRIES],
        }
    }
}

impl Default for PageTableNode {
    fn default() -> Self {
        Self::new()
    }
}

/// 模拟的三级页表。
///
/// 使用 HashMap<u64, PageTableNode> 模拟物理内存中的页表页。
/// `root_ppn` 是根页表所在的物理页号。
pub struct Sv39PageTable {
    /// 物理页号 -> 页表节点
    nodes: HashMap<u64, PageTableNode>,
    /// 根页表的物理页号
    pub root_ppn: u64,
    /// 下一个可分配的物理页号（简易分配器）
    next_ppn: u64,
}

/// 翻译结果
#[derive(Debug, PartialEq)]
pub enum TranslateResult {
    Ok(u64),
    PageFault,
}

impl Sv39PageTable {
    pub fn new() -> Self {
        let mut pt = Self {
            nodes: HashMap::new(),
            root_ppn: 0x80000,
            next_ppn: 0x80001,
        };
        pt.nodes.insert(pt.root_ppn, PageTableNode::new());
        pt
    }

    /// 分配一个新的物理页并初始化为空页表节点，返回其 PPN。
    fn alloc_node(&mut self) -> u64 {
        let ppn = self.next_ppn;
        self.next_ppn += 1;
        self.nodes.insert(ppn, PageTableNode::new());
        ppn
    }

    /// 从 39 位虚拟地址中提取第 `level` 级的 VPN。
    ///
    /// - level=2: 取 bits [38:30]
    /// - level=1: 取 bits [29:21]
    /// - level=0: 取 bits [20:12]
    ///
    /// 提示：右移 (12 + level * 9) 位，然后与 0x1FF 做掩码。
    pub fn extract_vpn(va: u64, level: usize) -> usize {
        // TODO: 从虚拟地址中提取指定级别的 VPN 索引
        //todo!()
        ((va >> (12 + level * 9)) & 0x1FF) as usize
    }

    /// 建立从虚拟页到物理页的映射（4KB 页）。
    ///
    /// 参数：
    /// - `va`: 虚拟地址（会自动对齐到页边界）
    /// - `pa`: 物理地址（会自动对齐到页边界）
    /// - `flags`: 标志位（如 PTE_V | PTE_R | PTE_W）
    pub fn map_page(&mut self, va: u64, pa: u64, flags: u64) {
        // 步骤 1: 从根页表开始，逐级向下遍历到 Level 1
        // ppn 指向当前正在处理的页表节点（初始为根页表）
        let mut ppn = self.root_ppn;
        
        // 步骤 2: 遍历中间层级（Level 2 和 Level 1）
        // 这些层级的 PTE 指向下一级页表，不是最终的物理地址
        for level in [2, 1] {
            // 2.1: 从虚拟地址中提取当前层级的 VPN 索引
            // - Level 2: 取 bits [38:30]，共 9 位
            // - Level 1: 取 bits [29:21]，共 9 位
            let vpn = Self::extract_vpn(va, level);
            
            // 2.2: 获取当前页表节点中对应 VPN 的 PTE
            let current_pte = self.nodes.get(&ppn).unwrap().entries[vpn];
            
            // 2.3: 检查 PTE 是否有效（PTE_V 标志位）
            if current_pte & PTE_V == 0 {
                // PTE 无效，说明下一级页表尚未分配
                // 需要分配一个新的页表节点（惰性分配）
                let new_ppn = self.alloc_node();
                
                // 在当前 PTE 中写入新页表的 PPN，并设置 PTE_V 标志
                // 格式: [PPN << 10 | Flags]，这里只设置 PTE_V
                self.nodes.get_mut(&ppn).unwrap().entries[vpn] = 
                    (new_ppn << PPN_SHIFT) | PTE_V;
                
                // 更新 ppn，指向新分配的页表节点，继续下一层级
                ppn = new_ppn;
            } else {
                // PTE 已有效，提取其中存储的下一级页表 PPN
                // PPN 位于 PTE 的 bits [63:10]，需要右移 10 位
                // & 0x7FFFFFF 确保只保留有效的 27 位 PPN
                ppn = (current_pte >> PPN_SHIFT) & 0x7FFFFFF;
            }
        }
        
        // 步骤 3: 到达 Level 0，写入最终的 VA -> PA 映射
        // 3.1: 提取 Level 0 的 VPN 索引（bits [20:12]）
        let vpn = Self::extract_vpn(va, 0);
        
        // 3.2: 在 Level 0 的 PTE 中写入物理页号和权限标志
        // - (pa >> 12): 将物理地址转换为物理页号（PPN）
        // - << PPN_SHIFT: 将 PPN 移到 PTE 的高位（bits [63:10]）
        // - | flags: 添加读/写/执行权限标志
        // - | PTE_V: 标记页表项为有效
        self.nodes.get_mut(&ppn).unwrap().entries[vpn] = 
            (pa >> 12) << PPN_SHIFT | flags | PTE_V;
    }

    /// 遍历三级页表，将虚拟地址翻译为物理地址。
    ///
    /// 步骤：
    /// 1. 从根页表（root_ppn）开始
    /// 2. 对每一级（2, 1, 0）：
    ///    a. 用 VPN[level] 索引当前页表节点
    ///    b. 如果 PTE 无效（!PTE_V），返回 PageFault
    ///    c. 如果 PTE 是叶节点（R|W|X 有任一置位），提取 PPN 计算物理地址
    ///    d. 否则用 PTE 中的 PPN 进入下一级页表
    /// 3. level 0 的 PTE 必须是叶节点
    pub fn translate(&self, va: u64) -> TranslateResult {
        // TODO: 实现三级页表遍历
        //
        // 提示：你需要从根页表开始，按 level 2 → level 1 → level 0 的顺序逐级遍历。
        // 每一级都需要通过 VPN[level] 索引当前页表节点的条目（PTE）。
        // 如果 PTE 无效（PTE_V == 0）则产生页错误（PageFault）。
        // 如果 PTE 是叶节点（即 R、W、X 标志位中有至少一个被置位），则可以直接使用该 PTE 中的物理页号（PPN）计算最终的物理地址。
        // 否则，该 PTE 指向下一级页表节点，继续遍历下一级。
        // 遍历到 level 0 时，PTE 必须是叶节点。
        //todo!()
        let mut ppn = self.root_ppn;
        
        // 步骤 2: 逐级遍历页表（Level 2 → Level 1 → Level 0）
        for level in [2, 1, 0] {
            // 2.1: 提取当前层级的 VPN 索引
            // Level 2 -> bits [38:30], Level 1 -> bits [29:21], Level 0 -> bits [20:12]
            let vpn = Self::extract_vpn(va, level);
            
            // 2.2: 获取当前页表节点
            let node = self.nodes.get(&ppn).unwrap();
            
            // 2.3: 读取对应 VPN 索引的页表项（PTE）
            let current_pte = node.entries[vpn];
            
            // 2.4: 检查 PTE 的有效性（Valid 位）
            // PTE_V = bit 0，表示该页表项是否有效
            if current_pte & PTE_V == 0 {
                // PTE 无效：页表项未被分配或映射
                // 返回页错误，表示虚拟地址未被映射
                return TranslateResult::PageFault;
            }
            
            // 2.5: 检查 PTE 是否为叶节点
            // R/W/X 标志位任一置位，表示该 PTE 是叶节点
            // 叶节点直接包含物理页号，而非指向下一级页表
            if current_pte & (PTE_R | PTE_W | PTE_X) != 0 {
                // 找到叶节点！计算物理地址
                
                // 2.5.1: 从 PTE 中提取物理页号（PPN）
                // PPN 位于 PTE 的 bits [63:10]
                // >> PPN_SHIFT 右移 10 位，& 0x7FFFFFF 只保留 27 位有效 PPN
                let ppn = (current_pte >> PPN_SHIFT) & 0x7FFFFFF;
                
                // 2.5.2: 计算页内偏移
                // - Level 1 叶节点（2MB 大页）: offset = va & 0x1FFFFF (21 bits)
                // - Level 0 叶节点（4KB 普通页）: offset = va & 0xFFF (12 bits)
                // 大页的 offset 更多，因为大页包含更多的虚拟地址空间
                let offset = if level == 1 { va & 0x1FFFFF } else { va & 0xFFF };
                
                // 2.5.3: 合成物理地址
                // 物理地址 = PPN * 页大小 + offset = (ppn << 12) | offset
                return TranslateResult::Ok((ppn << 12) | offset);
            }
            
            // 2.6: 当前 PTE 是中间节点（指向下一级页表）
            // 提取其中的 PPN，进入下一级页表继续查找
            ppn = (current_pte >> PPN_SHIFT) & 0x7FFFFFF;
        }
        
        // 正常情况下不会到达这里（Level 0 必须是叶节点）
        // 如果到达，说明页表结构异常
        TranslateResult::PageFault
    }

    /// 建立大页映射（2MB superpage，在 level 1 设叶子 PTE）。
    ///
    /// 2MB = 512 × 4KB，对齐要求：va 和 pa 都必须 2MB 对齐。
    ///
    /// 与 map_page 类似，但只遍历到 level 1 就写入叶子 PTE。
    pub fn map_superpage(&mut self, va: u64, pa: u64, flags: u64) {
        // 步骤 1: 验证对齐要求
        // 大页（2MB）必须 2MB 对齐，这是 RISC-V 硬件强制要求
        // mega_size = PAGE_SIZE * PT_ENTRIES = 4096 * 512 = 2097152 = 0x200000 (2MB)
        let mega_size: u64 = (PAGE_SIZE * PT_ENTRIES) as u64;
        assert_eq!(va % mega_size, 0, "va must be 2MB-aligned");
        assert_eq!(pa % mega_size, 0, "pa must be 2MB-aligned");

        // TODO: 实现大页映射
        //
        // 提示：大页映射与普通页映射类似，但只需要遍历到 level 1。
        // 你需要在 level 2 找到或创建中间页表节点，然后在 level 1 写入叶子 PTE。
        // 注意大页的物理页号计算方式与普通页相同（pa >> 12），
        // 但翻译时 offset 包含虚拟地址的低 21 位（VPN[0] 部分 + 12 位页内偏移）。
        let mut ppn = self.root_ppn;
        
        // 2.1: 提取 Level 2 的 VPN 索引（VA 的 bits [38:30]）
        let vpn_l2 = Self::extract_vpn(va, 2);
        
        // 2.2: 读取 Level 2 页表中对应 VPN 的 PTE
        let current_pte = self.nodes.get(&ppn).unwrap().entries[vpn_l2];
        
        // 2.3: 检查 PTE 是否有效，进行惰性分配
        if current_pte & PTE_V == 0 {
            // PTE 无效，分配一个新的 Level 1 页表节点
            let new_ppn = self.alloc_node();
            
            // 在 Level 2 的 PTE 中写入新 Level 1 页表的 PPN，并设置 PTE_V
            self.nodes.get_mut(&ppn).unwrap().entries[vpn_l2] = 
                (new_ppn << PPN_SHIFT) | PTE_V;
            
            // 更新 ppn 指向新分配的 Level 1 页表
            ppn = new_ppn;
        } else {
            // PTE 有效，提取其中存储的 Level 1 页表 PPN
            ppn = (current_pte >> PPN_SHIFT) & 0x7FFFFFF;
        }
        
        // 步骤 3: 在 Level 1 写入大页叶 PTE
        // 
        // 大页与普通页的关键区别：
        // - 普通页: 在 Level 0 写入叶 PTE，4KB 映射
        // - 大页:   在 Level 1 写入叶 PTE，2MB 映射
        //
        // Level 1 的一个 PTE 覆盖 512 个连续的 4KB 页 = 2MB
        // 这 512 个页共享同一个 PPN，但各自的 VPN[0] 不同
        
        // 3.1: 提取 Level 1 的 VPN 索引（VA 的 bits [29:21]）
        let vpn_l1 = Self::extract_vpn(va, 1);
        
        // 3.2: 在 Level 1 的 PTE 中写入大页的物理页号和权限标志
        // - (pa >> 12): 物理地址转换为物理页号
        // - << PPN_SHIFT: 移到 PTE 高位
        // - | flags: 添加 R/W/X 权限
        // - | PTE_V: 标记为有效（且是叶节点，因为设置了 R/W/X）
        self.nodes.get_mut(&ppn).unwrap().entries[vpn_l1] = 
            (pa >> 12) << PPN_SHIFT | flags | PTE_V;
        
        // 注意: 不再继续到 Level 0！Level 1 的叶 PTE 直接映射 2MB 区域
    }
}

impl Default for Sv39PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_vpn() {
        // VA = 0x0000_003F_FFFF_F000 (最大的 39 位地址的页边界)
        // VPN[2] = 0xFF (bits 38:30)
        // VPN[1] = 0x1FF (bits 29:21)
        // VPN[0] = 0x1FF (bits 20:12)
        let va: u64 = 0x7FFFFFF000;
        assert_eq!(Sv39PageTable::extract_vpn(va, 2), 0x1FF);
        assert_eq!(Sv39PageTable::extract_vpn(va, 1), 0x1FF);
        assert_eq!(Sv39PageTable::extract_vpn(va, 0), 0x1FF);
    }

    #[test]
    fn test_extract_vpn_simple() {
        // VA = 0x00000000 + page 1 = 0x1000
        // VPN[2] = 0, VPN[1] = 0, VPN[0] = 1
        let va: u64 = 0x1000;
        assert_eq!(Sv39PageTable::extract_vpn(va, 2), 0);
        assert_eq!(Sv39PageTable::extract_vpn(va, 1), 0);
        assert_eq!(Sv39PageTable::extract_vpn(va, 0), 1);
    }

    #[test]
    fn test_extract_vpn_level2() {
        // VPN[2] = 1 means bit 30 set -> VA >= 0x40000000
        let va: u64 = 0x40000000;
        assert_eq!(Sv39PageTable::extract_vpn(va, 2), 1);
        assert_eq!(Sv39PageTable::extract_vpn(va, 1), 0);
        assert_eq!(Sv39PageTable::extract_vpn(va, 0), 0);
    }

    #[test]
    fn test_map_and_translate_single() {
        let mut pt = Sv39PageTable::new();
        // 映射：VA 0x1000 -> PA 0x80001000
        pt.map_page(0x1000, 0x80001000, PTE_V | PTE_R);

        let result = pt.translate(0x1000);
        assert_eq!(result, TranslateResult::Ok(0x80001000));
    }

    #[test]
    fn test_translate_with_offset() {
        let mut pt = Sv39PageTable::new();
        pt.map_page(0x2000, 0x90000000, PTE_V | PTE_R | PTE_W);

        // 访问 VA 0x2ABC -> PA 应为 0x90000ABC
        let result = pt.translate(0x2ABC);
        assert_eq!(result, TranslateResult::Ok(0x90000ABC));
    }

    #[test]
    fn test_translate_page_fault() {
        let pt = Sv39PageTable::new();
        assert_eq!(pt.translate(0x1000), TranslateResult::PageFault);
    }

    #[test]
    fn test_multiple_mappings() {
        let mut pt = Sv39PageTable::new();
        pt.map_page(0x0000_1000, 0x8000_1000, PTE_V | PTE_R);
        pt.map_page(0x0000_2000, 0x8000_5000, PTE_V | PTE_R | PTE_W);
        pt.map_page(0x0040_0000, 0x9000_0000, PTE_V | PTE_R);

        assert_eq!(pt.translate(0x1234), TranslateResult::Ok(0x80001234));
        assert_eq!(pt.translate(0x2000), TranslateResult::Ok(0x80005000));
        assert_eq!(pt.translate(0x400100), TranslateResult::Ok(0x90000100));
    }

    #[test]
    fn test_map_overwrite() {
        let mut pt = Sv39PageTable::new();
        pt.map_page(0x1000, 0x80001000, PTE_V | PTE_R);
        assert_eq!(pt.translate(0x1000), TranslateResult::Ok(0x80001000));

        pt.map_page(0x1000, 0x90002000, PTE_V | PTE_R);
        assert_eq!(pt.translate(0x1000), TranslateResult::Ok(0x90002000));
    }

    #[test]
    fn test_superpage_mapping() {
        let mut pt = Sv39PageTable::new();
        // 2MB 大页映射：VA 0x200000 -> PA 0x80200000
        pt.map_superpage(0x200000, 0x80200000, PTE_V | PTE_R | PTE_W);

        // 大页内不同偏移都应命中
        assert_eq!(pt.translate(0x200000), TranslateResult::Ok(0x80200000));
        assert_eq!(pt.translate(0x200ABC), TranslateResult::Ok(0x80200ABC));
        assert_eq!(pt.translate(0x2FF000), TranslateResult::Ok(0x802FF000));
    }

    #[test]
    fn test_superpage_and_normal_coexist() {
        let mut pt = Sv39PageTable::new();
        // 大页映射在第一个 2MB 区域
        pt.map_superpage(0x0, 0x80000000, PTE_V | PTE_R);
        // 普通页在不同的 VPN[2] 区域
        pt.map_page(0x40000000, 0x90001000, PTE_V | PTE_R);

        assert_eq!(pt.translate(0x100), TranslateResult::Ok(0x80000100));
        assert_eq!(pt.translate(0x40000000), TranslateResult::Ok(0x90001000));
    }
}
