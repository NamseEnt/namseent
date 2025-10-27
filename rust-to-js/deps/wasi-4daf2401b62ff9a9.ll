; ModuleID = 'wasi.476d268f7a32d6a5-cgu.0'
source_filename = "wasi.476d268f7a32d6a5-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-wasi"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { ptr, [1 x i32] }

@alloc_ed8641ebea8e5515740d4eb49a916ff5 = private unnamed_addr constant [218 x i8] c"unsafe precondition(s) violated: ptr::read requires that the pointer argument is aligned and non-null\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_75fb06c2453febd814e73f5f2e72ae38 = private unnamed_addr constant [199 x i8] c"unsafe precondition(s) violated: hint::unreachable_unchecked must never be reached\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_86a8c10f7fc85c93d1bc5807d3c512ac = private unnamed_addr constant [122 x i8] c"/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasi-0.11.1+wasi-snapshot-preview1/src/lib_generated.rs\00", align 1
@alloc_67e747fb881d0fb15b51613bd08b794b = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00M\06\00\00\11\00\00\00" }>, align 4
@alloc_c6ac80460b9305d5066a706b621b8266 = private unnamed_addr constant [3 x i8] c"DIR", align 1
@alloc_f06bcdf8814ea766c9e9862e5f212d96 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\92\04\00\00\1B\00\00\00" }>, align 4
@alloc_b9865f014425c9240a60a0593d75bfd4 = private unnamed_addr constant [23 x i8] c"A pre-opened directory.", align 1
@alloc_bf1d48e0a063db09479fdcde4ace0db2 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\98\04\00\00\1B\00\00\00" }>, align 4
@alloc_d1f5559e557746700ee88ca153e6cc95 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\D6\07\00\00\11\00\00\00" }>, align 4
@alloc_d1bb501f48ae65fbc75ae2ac47791aa5 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\1F\08\00\00\11\00\00\00" }>, align 4
@alloc_ee9b84ec0de99e0eb905f593bf15a9be = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\06\05\00\00\11\00\00\00" }>, align 4
@alloc_f4d83df641848652ca5a0ee68d0f1a46 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00f\05\00\00\11\00\00\00" }>, align 4
@alloc_2710d116b7562e6e88e5f569ce3b3314 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00g\07\00\00\11\00\00\00" }>, align 4
@alloc_519248ce08ded2655ada4603f97e07fa = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\D0\04\00\00\0D\00\00\00" }>, align 4
@alloc_b1a543cfa155ac4cf241ff157a991f7d = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\D1\04\00\00\0D\00\00\00" }>, align 4
@alloc_123b1cf35884e2572af9f60d83db2bd9 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\1E\05\00\00\11\00\00\00" }>, align 4
@alloc_7d502150ec9455fc6ba4e8bf98dc17d0 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\E8\05\00\00\11\00\00\00" }>, align 4
@alloc_11b7520103d053307d60012511f4ff54 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\98\05\00\00\11\00\00\00" }>, align 4
@alloc_e122aba4a1aab03052298bf45e3e24db = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\EF\04\00\00\0D\00\00\00" }>, align 4
@alloc_7c8f1f6de8ed387b2a927003a7b97c49 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\F0\04\00\00\0D\00\00\00" }>, align 4
@alloc_ae2bd5de5116fcab94796dd68c9bb77c = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\D3\06\00\00\11\00\00\00" }>, align 4
@alloc_1713696202b080e762d75726f96ce4bd = private unnamed_addr constant [7 x i8] c"SUCCESS", align 1
@alloc_71a2b09e7c042f78ec615d7f2ca98c1b = private unnamed_addr constant [4 x i8] c"2BIG", align 1
@alloc_4981db4d4af768681fc505432fb27604 = private unnamed_addr constant [5 x i8] c"ACCES", align 1
@alloc_679cc6e775da6c924f4730aef593815b = private unnamed_addr constant [9 x i8] c"ADDRINUSE", align 1
@alloc_e46aa89b757d2be1eb1cb75b417933df = private unnamed_addr constant [12 x i8] c"ADDRNOTAVAIL", align 1
@alloc_d9627941932eaa106b478dfe15f39b87 = private unnamed_addr constant [11 x i8] c"AFNOSUPPORT", align 1
@alloc_c2f6cff0c49c8796b95e84de6cf7193f = private unnamed_addr constant [5 x i8] c"AGAIN", align 1
@alloc_c74b81658384824d0c279164dac5652c = private unnamed_addr constant [7 x i8] c"ALREADY", align 1
@alloc_e189066c7ac3daf6ee8ec65942018cc7 = private unnamed_addr constant [4 x i8] c"BADF", align 1
@alloc_ca66d8218ea61ea49e67cd560b7a6765 = private unnamed_addr constant [6 x i8] c"BADMSG", align 1
@alloc_b5396060cb2bc4d6a8892b062bdaa563 = private unnamed_addr constant [4 x i8] c"BUSY", align 1
@alloc_b5e9df1b543249147682414a6f297f1f = private unnamed_addr constant [8 x i8] c"CANCELED", align 1
@alloc_7e757e385d3a69379f474707734cf7ae = private unnamed_addr constant [5 x i8] c"CHILD", align 1
@alloc_466b9e6b1a6abc26631d981143acb197 = private unnamed_addr constant [11 x i8] c"CONNABORTED", align 1
@alloc_7475bd4428214e92b4dc1203d04ce21a = private unnamed_addr constant [11 x i8] c"CONNREFUSED", align 1
@alloc_5680db10ffd639594b86a5171e1323e2 = private unnamed_addr constant [9 x i8] c"CONNRESET", align 1
@alloc_9e91c6ecbddbf8ac8b47f0d0cfe27259 = private unnamed_addr constant [6 x i8] c"DEADLK", align 1
@alloc_e45387ccd4e83fb3e9151648d02011b7 = private unnamed_addr constant [11 x i8] c"DESTADDRREQ", align 1
@alloc_39ff86cee76cddbe698233f11e125157 = private unnamed_addr constant [3 x i8] c"DOM", align 1
@alloc_7532bc440e2a5cd291db16840aeb436b = private unnamed_addr constant [5 x i8] c"DQUOT", align 1
@alloc_4b36768a1ccde10a4d74de32c7566dae = private unnamed_addr constant [5 x i8] c"EXIST", align 1
@alloc_f47d5f6d9505f19073a40da4f201e89e = private unnamed_addr constant [5 x i8] c"FAULT", align 1
@alloc_d2a082495eb1a324c2165af137b6713f = private unnamed_addr constant [4 x i8] c"FBIG", align 1
@alloc_e4472dd01e3df0559c103997859ede93 = private unnamed_addr constant [11 x i8] c"HOSTUNREACH", align 1
@alloc_820fc975c3e424e90b13ff5fd907362d = private unnamed_addr constant [4 x i8] c"IDRM", align 1
@alloc_8b2a908f67e94e7bbbaa8522b21febe6 = private unnamed_addr constant [5 x i8] c"ILSEQ", align 1
@alloc_0e8392c5a03b89002a957a60556e615e = private unnamed_addr constant [10 x i8] c"INPROGRESS", align 1
@alloc_d32891f9920c2db7cf73e87c7574e15e = private unnamed_addr constant [4 x i8] c"INTR", align 1
@alloc_a5fe43a0c9a1b5894117e41abbec297f = private unnamed_addr constant [5 x i8] c"INVAL", align 1
@alloc_a9eaa09b855740b7afcf8c401274badc = private unnamed_addr constant [2 x i8] c"IO", align 1
@alloc_0a76b23954277669a40931fd8b7a3c0b = private unnamed_addr constant [6 x i8] c"ISCONN", align 1
@alloc_48dd9fbc0aa0a1ec0bd01e76772188ca = private unnamed_addr constant [5 x i8] c"ISDIR", align 1
@alloc_dd63b73adf6e9192eed7882c1e0dccda = private unnamed_addr constant [4 x i8] c"LOOP", align 1
@alloc_79ff279bd852f075db2122f2b968aef1 = private unnamed_addr constant [5 x i8] c"MFILE", align 1
@alloc_7438b4b6de0425646bb9362129be2540 = private unnamed_addr constant [5 x i8] c"MLINK", align 1
@alloc_08cd9c903b35ceb9437506b4b82226a7 = private unnamed_addr constant [7 x i8] c"MSGSIZE", align 1
@alloc_b67a0498ec853dd0b8fd56ca21603195 = private unnamed_addr constant [8 x i8] c"MULTIHOP", align 1
@alloc_8d9481f0445b1a3759b069a16ddbc6b8 = private unnamed_addr constant [11 x i8] c"NAMETOOLONG", align 1
@alloc_edaf563e7ced42fe9a4b29da8944b938 = private unnamed_addr constant [7 x i8] c"NETDOWN", align 1
@alloc_33ba081697e0cbd20f7a3190e0e07ecd = private unnamed_addr constant [8 x i8] c"NETRESET", align 1
@alloc_52700a7601ce3574eb8f6a19c2a2f3b0 = private unnamed_addr constant [10 x i8] c"NETUNREACH", align 1
@alloc_b6df84a30618b409bf4463f51730701e = private unnamed_addr constant [5 x i8] c"NFILE", align 1
@alloc_7ad907da16c51d1b1767f589dbedb81c = private unnamed_addr constant [6 x i8] c"NOBUFS", align 1
@alloc_390128a259ca122ccc7c101503dcf7e2 = private unnamed_addr constant [5 x i8] c"NODEV", align 1
@alloc_77e32f5571452900dccaaddf2e771641 = private unnamed_addr constant [5 x i8] c"NOENT", align 1
@alloc_672d3a2ca4a906d2c49343de26785b3d = private unnamed_addr constant [6 x i8] c"NOEXEC", align 1
@alloc_c2361c8fb3e1574e929b197bb6f642d9 = private unnamed_addr constant [5 x i8] c"NOLCK", align 1
@alloc_d34415974c96b1818039e07e24c9a542 = private unnamed_addr constant [6 x i8] c"NOLINK", align 1
@alloc_98446342a477dc4b70825be19fec689c = private unnamed_addr constant [5 x i8] c"NOMEM", align 1
@alloc_9d29ccf1851f76dea2b1eda56b1339a4 = private unnamed_addr constant [5 x i8] c"NOMSG", align 1
@alloc_338071ff59f25595243757f423cf8c4d = private unnamed_addr constant [10 x i8] c"NOPROTOOPT", align 1
@alloc_acb3dc91cb500f6df7d122691e000086 = private unnamed_addr constant [5 x i8] c"NOSPC", align 1
@alloc_0768e0e6701eea9bd662f098d8a76639 = private unnamed_addr constant [5 x i8] c"NOSYS", align 1
@alloc_aad7f4837a0f073d74c347d1aa856e5a = private unnamed_addr constant [7 x i8] c"NOTCONN", align 1
@alloc_cb80030390ab958e664277e7ca12fcff = private unnamed_addr constant [6 x i8] c"NOTDIR", align 1
@alloc_18d75c46ae0ab8f50e6836934fbf2600 = private unnamed_addr constant [8 x i8] c"NOTEMPTY", align 1
@alloc_5daae7153a002e51f143b30b78309845 = private unnamed_addr constant [14 x i8] c"NOTRECOVERABLE", align 1
@alloc_1be84d0a1f1bf46ad4dc0650fdee1233 = private unnamed_addr constant [7 x i8] c"NOTSOCK", align 1
@alloc_fa922dc79f516041a49296256e336569 = private unnamed_addr constant [6 x i8] c"NOTSUP", align 1
@alloc_35918dfdd30516e0576216d8bb37a7e8 = private unnamed_addr constant [5 x i8] c"NOTTY", align 1
@alloc_23e5ef0eb64cf9a4828eed9d2cdbb19f = private unnamed_addr constant [4 x i8] c"NXIO", align 1
@alloc_3b8b4793b1ea451345d29976e468891f = private unnamed_addr constant [8 x i8] c"OVERFLOW", align 1
@alloc_a060d5bdeb7be0aa9e9bc429ea28a915 = private unnamed_addr constant [9 x i8] c"OWNERDEAD", align 1
@alloc_c6663eba2feb4c0fa6a6601ac4c9f961 = private unnamed_addr constant [4 x i8] c"PERM", align 1
@alloc_4832908c0d36609818b44d52da3b4ab1 = private unnamed_addr constant [4 x i8] c"PIPE", align 1
@alloc_39701fd8446760e713828a22e8e84657 = private unnamed_addr constant [5 x i8] c"PROTO", align 1
@alloc_0fb76d18eb4a8293863d42d53c0164bc = private unnamed_addr constant [14 x i8] c"PROTONOSUPPORT", align 1
@alloc_f0ee6b554743936719182439aea5a47f = private unnamed_addr constant [9 x i8] c"PROTOTYPE", align 1
@alloc_f1b4a2fb1dc3d6723575e33bd5591258 = private unnamed_addr constant [5 x i8] c"RANGE", align 1
@alloc_1a181a492552b27e757c957fa98b4301 = private unnamed_addr constant [4 x i8] c"ROFS", align 1
@alloc_f36662d78f18dcd86d6806ad83e3d270 = private unnamed_addr constant [5 x i8] c"SPIPE", align 1
@alloc_a512a652cf8a4f93826461fecbbde9de = private unnamed_addr constant [4 x i8] c"SRCH", align 1
@alloc_8118216a9976010c8fb26905b45815b0 = private unnamed_addr constant [5 x i8] c"STALE", align 1
@alloc_b79ec72229df8c2bb729b17b46549851 = private unnamed_addr constant [8 x i8] c"TIMEDOUT", align 1
@alloc_3dbf8c65f045ddaf28c14ea28745ff8b = private unnamed_addr constant [6 x i8] c"TXTBSY", align 1
@alloc_cbc52d0443417071f78111a521226f88 = private unnamed_addr constant [4 x i8] c"XDEV", align 1
@alloc_98b30359d473d22b9e60dcaa28fd9cc0 = private unnamed_addr constant [10 x i8] c"NOTCAPABLE", align 1
@alloc_f41208f7f9aabadef03af3b7c3757409 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\004\01\00\00\1B\00\00\00" }>, align 4
@alloc_738b72084b53a4d34e6543f6fa0616e5 = private unnamed_addr constant [54 x i8] c"No error occurred. System call completed successfully.", align 1
@alloc_63cb38bf054ed0aa1c1989dca559a70d = private unnamed_addr constant [23 x i8] c"Argument list too long.", align 1
@alloc_6f0e46a1b1c28b0b5efd847e7c5099af = private unnamed_addr constant [18 x i8] c"Permission denied.", align 1
@alloc_6336caef1304c9177e345ebb2e043fda = private unnamed_addr constant [15 x i8] c"Address in use.", align 1
@alloc_019b84fb7e896b1e2ff05536dcbff498 = private unnamed_addr constant [22 x i8] c"Address not available.", align 1
@alloc_41a7c3ef62d536421e93c122f5fe7b8d = private unnamed_addr constant [29 x i8] c"Address family not supported.", align 1
@alloc_72667a1889bb1fb3adcae40a49439f4e = private unnamed_addr constant [47 x i8] c"Resource unavailable, or operation would block.", align 1
@alloc_c75ac68611722b9931e3c07c7d349ff9 = private unnamed_addr constant [31 x i8] c"Connection already in progress.", align 1
@alloc_50bf1fbfc329f4125b504796273167be = private unnamed_addr constant [20 x i8] c"Bad file descriptor.", align 1
@alloc_db7708960a5e94965c9dbdd9624fc4cb = private unnamed_addr constant [12 x i8] c"Bad message.", align 1
@alloc_f0bed62e43f778e5a8496b2c99a845d7 = private unnamed_addr constant [24 x i8] c"Device or resource busy.", align 1
@alloc_c4e6fe31be9e1422f6c2c65a1d48a6b3 = private unnamed_addr constant [19 x i8] c"Operation canceled.", align 1
@alloc_d14052a2d17b8b4721b366ca99170a7e = private unnamed_addr constant [19 x i8] c"No child processes.", align 1
@alloc_6cde56766d930cfca05b143c24c97a3c = private unnamed_addr constant [19 x i8] c"Connection aborted.", align 1
@alloc_f576af0d4e37b0b54eba3c15d93ddbd5 = private unnamed_addr constant [19 x i8] c"Connection refused.", align 1
@alloc_69a56768e5c93b3bcd705dc2bbf127c1 = private unnamed_addr constant [17 x i8] c"Connection reset.", align 1
@alloc_596999da3cde17096c9e2140c0918ba4 = private unnamed_addr constant [30 x i8] c"Resource deadlock would occur.", align 1
@alloc_e25cb539fdcd1152765745e7db037191 = private unnamed_addr constant [29 x i8] c"Destination address required.", align 1
@alloc_f457658de3d1562f99548ba5528655f8 = private unnamed_addr constant [47 x i8] c"Mathematics argument out of domain of function.", align 1
@alloc_d8f7503a36cdc9bc6346c9e92a42bc81 = private unnamed_addr constant [9 x i8] c"Reserved.", align 1
@alloc_f12c98ce38519e21b9c9da75f9f4fc43 = private unnamed_addr constant [12 x i8] c"File exists.", align 1
@alloc_a91e11b0ce836ee177ea52c8e7cd8d50 = private unnamed_addr constant [12 x i8] c"Bad address.", align 1
@alloc_c41cb924416df4aca21adddaa5bd21dd = private unnamed_addr constant [15 x i8] c"File too large.", align 1
@alloc_dfb69db467b1a3656fb6fdbace99c9b9 = private unnamed_addr constant [20 x i8] c"Host is unreachable.", align 1
@alloc_9b981c3df33594e259c917c223bfc7a6 = private unnamed_addr constant [19 x i8] c"Identifier removed.", align 1
@alloc_4bfe52500497b30818b54150a30d4f08 = private unnamed_addr constant [22 x i8] c"Illegal byte sequence.", align 1
@alloc_4f6557f2690f452d2e5554006942347f = private unnamed_addr constant [22 x i8] c"Operation in progress.", align 1
@alloc_d073b4ff65817f21be5ef28dd0a03cda = private unnamed_addr constant [21 x i8] c"Interrupted function.", align 1
@alloc_2d0f752c50db342d0d94862ec0dd55e9 = private unnamed_addr constant [17 x i8] c"Invalid argument.", align 1
@alloc_38bacb6f728efb49beca495dbc067ccf = private unnamed_addr constant [10 x i8] c"I/O error.", align 1
@alloc_c0362ec6ca12ceceb48261de984a7166 = private unnamed_addr constant [20 x i8] c"Socket is connected.", align 1
@alloc_31b18777694e6f001b4e1d2203fbab80 = private unnamed_addr constant [15 x i8] c"Is a directory.", align 1
@alloc_bd5d88c6439a3c98342310082da1f1c2 = private unnamed_addr constant [34 x i8] c"Too many levels of symbolic links.", align 1
@alloc_9b00bf851dceae7b81b26420d02dd5f2 = private unnamed_addr constant [32 x i8] c"File descriptor value too large.", align 1
@alloc_83d8f44faafe130e1396daa0c5b148d0 = private unnamed_addr constant [15 x i8] c"Too many links.", align 1
@alloc_26338639bc7c94a746aa95f550572308 = private unnamed_addr constant [18 x i8] c"Message too large.", align 1
@alloc_1f1653a1bd7a0c91ce65b8bdfb6e14d3 = private unnamed_addr constant [18 x i8] c"Filename too long.", align 1
@alloc_5aea10ec52d9b12f8025d677170427ae = private unnamed_addr constant [16 x i8] c"Network is down.", align 1
@alloc_4737f61e08be7fd4781363eaf0ba41f3 = private unnamed_addr constant [30 x i8] c"Connection aborted by network.", align 1
@alloc_42fe82e120ce05d367941be8c45758d6 = private unnamed_addr constant [20 x i8] c"Network unreachable.", align 1
@alloc_210a08b40d17b5afdf1083a0818a09c2 = private unnamed_addr constant [30 x i8] c"Too many files open in system.", align 1
@alloc_ea505343c96a58ff92689d83b2ef7605 = private unnamed_addr constant [26 x i8] c"No buffer space available.", align 1
@alloc_f8ce4cd55bfbd4ae871da2f1a4336f45 = private unnamed_addr constant [15 x i8] c"No such device.", align 1
@alloc_aff65f0f6f2c87a1347784f12b25642c = private unnamed_addr constant [26 x i8] c"No such file or directory.", align 1
@alloc_bcc1ecb9294c681d33e8c041b1bcbfbf = private unnamed_addr constant [29 x i8] c"Executable file format error.", align 1
@alloc_2c388975459832c1da18b16bb6f6cad7 = private unnamed_addr constant [19 x i8] c"No locks available.", align 1
@alloc_9cc3052ce766bc8d1bd2f0472f25d6ed = private unnamed_addr constant [17 x i8] c"Not enough space.", align 1
@alloc_c5d465d10be95607dc340c7a4d24f2bd = private unnamed_addr constant [31 x i8] c"No message of the desired type.", align 1
@alloc_6ceecfbc50ac588c65f787ca7566721e = private unnamed_addr constant [23 x i8] c"Protocol not available.", align 1
@alloc_ad22dde9eb788be408dfbce5b536ecb1 = private unnamed_addr constant [24 x i8] c"No space left on device.", align 1
@alloc_a5c36ad2f7f39f8c571bed384c4aa86e = private unnamed_addr constant [23 x i8] c"Function not supported.", align 1
@alloc_79e5afbb2836e42f1cf6545735def723 = private unnamed_addr constant [28 x i8] c"The socket is not connected.", align 1
@alloc_b79b88e1517127bd26a11caf03e96a4b = private unnamed_addr constant [50 x i8] c"Not a directory or a symbolic link to a directory.", align 1
@alloc_5bd796a2729b29dc1077aa78010540eb = private unnamed_addr constant [20 x i8] c"Directory not empty.", align 1
@alloc_2227009005ed3aef895a625a934fb490 = private unnamed_addr constant [22 x i8] c"State not recoverable.", align 1
@alloc_7b2218a5156569098ceb294456ea6101 = private unnamed_addr constant [13 x i8] c"Not a socket.", align 1
@alloc_ee0d04358a5a06f6330667e5324932c2 = private unnamed_addr constant [52 x i8] c"Not supported, or operation not supported on socket.", align 1
@alloc_e6d870eee4288e15aaacc1f81fe3252c = private unnamed_addr constant [36 x i8] c"Inappropriate I/O control operation.", align 1
@alloc_b33f52b7e7f121d0c226151f5c778cf3 = private unnamed_addr constant [26 x i8] c"No such device or address.", align 1
@alloc_5ae69e8e195aceb61f3f3c18eb8bcf47 = private unnamed_addr constant [42 x i8] c"Value too large to be stored in data type.", align 1
@alloc_2ea23f84b1b3d5f1a01ca6aee02ddfe3 = private unnamed_addr constant [20 x i8] c"Previous owner died.", align 1
@alloc_7541958cba33228c76ccfd6ef3a998f8 = private unnamed_addr constant [24 x i8] c"Operation not permitted.", align 1
@alloc_267ce7f25696333bd0d66667a08bb4cc = private unnamed_addr constant [12 x i8] c"Broken pipe.", align 1
@alloc_651051b66cd9fca775b419eff6fac487 = private unnamed_addr constant [15 x i8] c"Protocol error.", align 1
@alloc_34caa06c16574cedbfc891de933cd74b = private unnamed_addr constant [23 x i8] c"Protocol not supported.", align 1
@alloc_e4d49fcb16c811e138f85e7abb11a382 = private unnamed_addr constant [31 x i8] c"Protocol wrong type for socket.", align 1
@alloc_05506144d01cb8b7afa4dd1845c86028 = private unnamed_addr constant [17 x i8] c"Result too large.", align 1
@alloc_802a116d829d4cbb92a657c6226a6057 = private unnamed_addr constant [22 x i8] c"Read-only file system.", align 1
@alloc_b6a7d25ded2c9ed637d0c584657272ef = private unnamed_addr constant [13 x i8] c"Invalid seek.", align 1
@alloc_40faf286812d0e936efdc3d065b7fc25 = private unnamed_addr constant [16 x i8] c"No such process.", align 1
@alloc_39b15d988456c8fcdc341a0d6e07d435 = private unnamed_addr constant [21 x i8] c"Connection timed out.", align 1
@alloc_095b6ceaeb015f29eb0c009fbd12eb42 = private unnamed_addr constant [15 x i8] c"Text file busy.", align 1
@alloc_6dbbe3e0d2d606ce792f0527a3d75325 = private unnamed_addr constant [18 x i8] c"Cross-device link.", align 1
@alloc_090e2e52fa078921cfe166d26b6497f7 = private unnamed_addr constant [37 x i8] c"Extension: Capabilities insufficient.", align 1
@alloc_92339703776781697a7015ea2de787b5 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\86\01\00\00\1B\00\00\00" }>, align 4
@alloc_3eb08fd558da99ff59a728fa22216608 = private unnamed_addr constant [6 x i8] c"NORMAL", align 1
@alloc_c6c5eb2716d59a1e76b4d880a95c1203 = private unnamed_addr constant [10 x i8] c"SEQUENTIAL", align 1
@alloc_fc1d752d3184d907ca6eeb15408fb425 = private unnamed_addr constant [6 x i8] c"RANDOM", align 1
@alloc_65a421ac3660e213e6276b3729b0fb05 = private unnamed_addr constant [8 x i8] c"WILLNEED", align 1
@alloc_2b00a549833884ad7baf6eabe3256f4c = private unnamed_addr constant [8 x i8] c"DONTNEED", align 1
@alloc_9ee9b0fb51e102c8b3d02ce7e5369dcf = private unnamed_addr constant [7 x i8] c"NOREUSE", align 1
@alloc_0bce313c2a20451cabb54b4d3072bba5 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\83\02\00\00\1B\00\00\00" }>, align 4
@alloc_2f1b7feb59b741ed0dcaa6768be38f92 = private unnamed_addr constant [89 x i8] c"The application has no advice to give on its behavior with respect to the specified data.", align 1
@alloc_cae59b4057e77128fda7a33586d57565 = private unnamed_addr constant [103 x i8] c"The application expects to access the specified data sequentially from lower offsets to higher offsets.", align 1
@alloc_7379a0ec492be39e3d90d8d9194bce4b = private unnamed_addr constant [71 x i8] c"The application expects to access the specified data in a random order.", align 1
@alloc_46b81c5cab4937138c3046baf3ce0eaa = private unnamed_addr constant [72 x i8] c"The application expects to access the specified data in the near future.", align 1
@alloc_c98966898364094a7765a5dee620d7b7 = private unnamed_addr constant [86 x i8] c"The application expects that it will not access the specified data in the near future.", align 1
@alloc_33a9062b2e552fbd5532b3c7655622a8 = private unnamed_addr constant [91 x i8] c"The application expects to access the specified data once and then not reuse it thereafter.", align 1
@alloc_28e1352c6e0fbf0afa06b76f00de61e7 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\87\02\00\00U\02\00\00" }>, align 4
@alloc_7b2f6f7016e80840b06d869ab1ff6b9c = private unnamed_addr constant [4 x i8] c"NONE", align 1
@alloc_5305d1a6cfb98272599f584034cb1d99 = private unnamed_addr constant [3 x i8] c"HUP", align 1
@alloc_9ecfdfa2c73d42c419fcbbe6557cd56c = private unnamed_addr constant [3 x i8] c"INT", align 1
@alloc_006b3ed861a34a316b91090b379496e8 = private unnamed_addr constant [4 x i8] c"QUIT", align 1
@alloc_2c150e288be0a9bb436351adfd41c311 = private unnamed_addr constant [3 x i8] c"ILL", align 1
@alloc_e01af9736b524006e5fb45ad54018c38 = private unnamed_addr constant [4 x i8] c"TRAP", align 1
@alloc_d347cac5445f7cb2d7b668ea9b684875 = private unnamed_addr constant [4 x i8] c"ABRT", align 1
@alloc_a467c9853a95bc3886e7dda7c3f32aae = private unnamed_addr constant [3 x i8] c"BUS", align 1
@alloc_172396a7e6a4caf1be590182cac51572 = private unnamed_addr constant [3 x i8] c"FPE", align 1
@alloc_80a21bb18cfbe63f6862ef423ec1eb36 = private unnamed_addr constant [4 x i8] c"KILL", align 1
@alloc_07a1d14791a230a5109bcf389757953c = private unnamed_addr constant [4 x i8] c"USR1", align 1
@alloc_9fae5217b901fa82c8a06bff12e6775f = private unnamed_addr constant [4 x i8] c"SEGV", align 1
@alloc_544a5f255a52c55972be2a457da1e695 = private unnamed_addr constant [4 x i8] c"USR2", align 1
@alloc_a2e52520915a0d17d404534001d33e86 = private unnamed_addr constant [4 x i8] c"ALRM", align 1
@alloc_c940f1872184b67533cde325d4eb7ceb = private unnamed_addr constant [4 x i8] c"TERM", align 1
@alloc_700eaf7a86f77d80c28b2adbf3cc9803 = private unnamed_addr constant [4 x i8] c"CHLD", align 1
@alloc_1e6c27aaa4a4f9cb003486cc93d37053 = private unnamed_addr constant [4 x i8] c"CONT", align 1
@alloc_916b433ff317c0fd3edfdd465d6ba8b9 = private unnamed_addr constant [4 x i8] c"STOP", align 1
@alloc_9cc6a38345689c915b47d594cb019fed = private unnamed_addr constant [4 x i8] c"TSTP", align 1
@alloc_c4577f07e7c9ccf92dd42115bc7131a5 = private unnamed_addr constant [4 x i8] c"TTIN", align 1
@alloc_32f6530f1dc9b98127bcb811b0532ba7 = private unnamed_addr constant [4 x i8] c"TTOU", align 1
@alloc_36c7b7b3d4a1a29180c38bf79b66c057 = private unnamed_addr constant [3 x i8] c"URG", align 1
@alloc_7eb7520302da06d239e69560c3e0708c = private unnamed_addr constant [4 x i8] c"XCPU", align 1
@alloc_40234d469372e9f8ef518dd54a19727f = private unnamed_addr constant [4 x i8] c"XFSZ", align 1
@alloc_31643e4a78119bdf0ff87de45542db06 = private unnamed_addr constant [6 x i8] c"VTALRM", align 1
@alloc_42106c4e8e5f57f6c471196129c7ada1 = private unnamed_addr constant [4 x i8] c"PROF", align 1
@alloc_eb9a91982746a8bf460d0a0d2c98fe81 = private unnamed_addr constant [5 x i8] c"WINCH", align 1
@alloc_d86c1c0ecbe01a4015494d5a0a7bd4e8 = private unnamed_addr constant [4 x i8] c"POLL", align 1
@alloc_1d9e67eff05b636a4dea07e14807617f = private unnamed_addr constant [3 x i8] c"PWR", align 1
@alloc_13d53d5a7d472d642bc2ba5eddd00818 = private unnamed_addr constant [3 x i8] c"SYS", align 1
@alloc_26b978e47b28a070f1ba411c19d37f13 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\E5\03\00\00\1B\00\00\00" }>, align 4
@alloc_1423cf9b1285dc2276463c9080968c6e = private unnamed_addr constant [95 x i8] c"No signal. Note that POSIX has special semantics for `kill(pid, 0)`,\0Aso this value is reserved.", align 1
@alloc_d1463c6d690f4195e6ae033efbc5e8aa = private unnamed_addr constant [39 x i8] c"Hangup.\0AAction: Terminates the process.", align 1
@alloc_41d2790e9dad417b78c2043eab3b612b = private unnamed_addr constant [59 x i8] c"Terminate interrupt signal.\0AAction: Terminates the process.", align 1
@alloc_5580ea2c7771dcb161701338a11bf185 = private unnamed_addr constant [53 x i8] c"Terminal quit signal.\0AAction: Terminates the process.", align 1
@alloc_248a2f19574abf8c31e56d6193a6bc66 = private unnamed_addr constant [52 x i8] c"Illegal instruction.\0AAction: Terminates the process.", align 1
@alloc_493a384016f4bf3e4c3889a7faf9e4e3 = private unnamed_addr constant [54 x i8] c"Trace/breakpoint trap.\0AAction: Terminates the process.", align 1
@alloc_ed92c62a78c492f3c7b01873f82aa31e = private unnamed_addr constant [53 x i8] c"Process abort signal.\0AAction: Terminates the process.", align 1
@alloc_84a64b8254125961372be19e65c94f49 = private unnamed_addr constant [82 x i8] c"Access to an undefined portion of a memory object.\0AAction: Terminates the process.", align 1
@alloc_80bf056227e060e13fa753e3e1de892d = private unnamed_addr constant [63 x i8] c"Erroneous arithmetic operation.\0AAction: Terminates the process.", align 1
@alloc_2c10eaca4ad1b29d69d589d9e9e296ac = private unnamed_addr constant [37 x i8] c"Kill.\0AAction: Terminates the process.", align 1
@alloc_356d3fe077f6c83805a2b19a2b21ae2f = private unnamed_addr constant [54 x i8] c"User-defined signal 1.\0AAction: Terminates the process.", align 1
@alloc_9f15daac07f269ee6fd44f89ce0f59bf = private unnamed_addr constant [57 x i8] c"Invalid memory reference.\0AAction: Terminates the process.", align 1
@alloc_689faaf1d42056dc8b2f636f12486583 = private unnamed_addr constant [54 x i8] c"User-defined signal 2.\0AAction: Terminates the process.", align 1
@alloc_2200f213787ac008059970ba784239a7 = private unnamed_addr constant [56 x i8] c"Write on a pipe with no one to read it.\0AAction: Ignored.", align 1
@alloc_2858c2305fa03fdc3d3c76407e2cc263 = private unnamed_addr constant [44 x i8] c"Alarm clock.\0AAction: Terminates the process.", align 1
@alloc_c3e8042022d30e3d9443605454f05952 = private unnamed_addr constant [51 x i8] c"Termination signal.\0AAction: Terminates the process.", align 1
@alloc_0a31d2078b214ff88d1994cf10051d5a = private unnamed_addr constant [65 x i8] c"Child process terminated, stopped, or continued.\0AAction: Ignored.", align 1
@alloc_745dc892bfb3506c1d83e31c0d775108 = private unnamed_addr constant [72 x i8] c"Continue executing, if stopped.\0AAction: Continues executing, if stopped.", align 1
@alloc_a59eb384144ffa9b0a65ee678a07f52c = private unnamed_addr constant [40 x i8] c"Stop executing.\0AAction: Stops executing.", align 1
@alloc_17a4a3403af04ca62d45bd3a94aa5655 = private unnamed_addr constant [46 x i8] c"Terminal stop signal.\0AAction: Stops executing.", align 1
@alloc_58995edd6c7ab0cd8cef2af5cecf15ef = private unnamed_addr constant [60 x i8] c"Background process attempting read.\0AAction: Stops executing.", align 1
@alloc_460288a5f6a2eec462f5597a92f2d641 = private unnamed_addr constant [61 x i8] c"Background process attempting write.\0AAction: Stops executing.", align 1
@alloc_45d2b94afd2e05ef565c22d943786616 = private unnamed_addr constant [62 x i8] c"High bandwidth data is available at a socket.\0AAction: Ignored.", align 1
@alloc_c2e111ecc720ba84b14250e38b86b16d = private unnamed_addr constant [56 x i8] c"CPU time limit exceeded.\0AAction: Terminates the process.", align 1
@alloc_74a3ad70801e06f3bfe7505d1789a8f4 = private unnamed_addr constant [57 x i8] c"File size limit exceeded.\0AAction: Terminates the process.", align 1
@alloc_c198d2f1aabfc2bab249933e1ee4eb1c = private unnamed_addr constant [54 x i8] c"Virtual timer expired.\0AAction: Terminates the process.", align 1
@alloc_20bb6bf3f1ace7525c9405f2282207b6 = private unnamed_addr constant [56 x i8] c"Profiling timer expired.\0AAction: Terminates the process.", align 1
@alloc_150b5f9befb6857dc7daa49e9e6fbc95 = private unnamed_addr constant [32 x i8] c"Window changed.\0AAction: Ignored.", align 1
@alloc_6ab3986896163e2ad59f1f57a6e1519f = private unnamed_addr constant [45 x i8] c"I/O possible.\0AAction: Terminates the process.", align 1
@alloc_a4c30caabbc8ee013e8bfac02e84c080 = private unnamed_addr constant [46 x i8] c"Power failure.\0AAction: Terminates the process.", align 1
@alloc_97b2e00b5705f2c9646cc82f6fb99c61 = private unnamed_addr constant [48 x i8] c"Bad system call.\0AAction: Terminates the process.", align 1
@alloc_53d68276a954c11b13aab7829171cada = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00f\04\00\00\1B\00\00\00" }>, align 4
@alloc_5db363c82b07540837cd27a17f0c663b = private unnamed_addr constant [3 x i8] c"SET", align 1
@alloc_ff6ccd04681148d223fbb1fb8e2efc21 = private unnamed_addr constant [3 x i8] c"CUR", align 1
@alloc_29db745e090ad0183dfdc88ed77b1433 = private unnamed_addr constant [3 x i8] c"END", align 1
@alloc_c07eba15eca988477b5fb63c67d540a8 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\0F\02\00\00\1B\00\00\00" }>, align 4
@alloc_24a3caf022c395e6ae7e098e8f399198 = private unnamed_addr constant [31 x i8] c"Seek relative to start-of-file.", align 1
@alloc_2c2e9693ef801993a434a363e8a8a714 = private unnamed_addr constant [34 x i8] c"Seek relative to current position.", align 1
@alloc_10ffe8fd38749369565ce0219086a5bd = private unnamed_addr constant [29 x i8] c"Seek relative to end-of-file.", align 1
@alloc_c7c92b698ef4610640186963f648efe9 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\17\02\00\00\1B\00\00\00" }>, align 4
@alloc_a902d1a7a762d9fe1e7935df74c14685 = private unnamed_addr constant [8 x i8] c"REALTIME", align 1
@alloc_b430f478269159bc9daab7abd375695f = private unnamed_addr constant [9 x i8] c"MONOTONIC", align 1
@alloc_2e6987dab4e18cfd64262ee6f5a1f05b = private unnamed_addr constant [18 x i8] c"PROCESS_CPUTIME_ID", align 1
@alloc_34477b3239657b53925e478a823eef9e = private unnamed_addr constant [17 x i8] c"THREAD_CPUTIME_ID", align 1
@alloc_8bbd04dca6dc62dbeef9758be427fde9 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00$\00\00\00\1B\00\00\00" }>, align 4
@alloc_1aea4d167b9a42393df22a71e3a8f11b = private unnamed_addr constant [85 x i8] c"The clock measuring real time. Time value zero corresponds with\0A1970-01-01T00:00:00Z.", align 1
@alloc_5bfa82a09c61ee9eec58df8e794b5e7a = private unnamed_addr constant [257 x i8] c"The store-wide monotonic clock, which is defined as a clock measuring\0Areal time, whose value cannot be adjusted and which cannot have negative\0Aclock jumps. The epoch of this clock is undefined. The absolute time\0Avalue of this clock therefore has no meaning.", align 1
@alloc_2d54b354caf70a3f2c540cc2dfd4a0b8 = private unnamed_addr constant [55 x i8] c"The CPU-time clock associated with the current process.", align 1
@alloc_e7d7dc92d73b7cdd8f44c10b5d3c455b = private unnamed_addr constant [54 x i8] c"The CPU-time clock associated with the current thread.", align 1
@alloc_e3f936e04555125f465e22d7f3bb2614 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\005\00\00\00\1B\00\00\00" }>, align 4
@alloc_70e39a2b9d2b22656a4b6352fc00abaa = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00'\06\00\00\11\00\00\00" }>, align 4
@alloc_ea3c79e2bbf0f7b9771412f9a1555d86 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00z\06\00\00\11\00\00\00" }>, align 4
@alloc_1364e2512203bc4925076d374c123589 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\93\06\00\00\11\00\00\00" }>, align 4
@alloc_3e9eb586b0e75631ded4bff06697b5ef = private unnamed_addr constant [7 x i8] c"UNKNOWN", align 1
@alloc_6ad3f50235942c9b42c0e7e5cb6f0f1c = private unnamed_addr constant [12 x i8] c"BLOCK_DEVICE", align 1
@alloc_2478ed23cecb242b65d611620b0117f3 = private unnamed_addr constant [16 x i8] c"CHARACTER_DEVICE", align 1
@alloc_9a62b518932bfe53ea52a4b3adc16526 = private unnamed_addr constant [9 x i8] c"DIRECTORY", align 1
@alloc_abfb6e9fa12f0cc71c3152a03fa9d825 = private unnamed_addr constant [12 x i8] c"REGULAR_FILE", align 1
@alloc_3ebef2482db8a977a988cb55208895b9 = private unnamed_addr constant [12 x i8] c"SOCKET_DGRAM", align 1
@alloc_2de38242575aec0efc9f447ec8601b4f = private unnamed_addr constant [13 x i8] c"SOCKET_STREAM", align 1
@alloc_34ce0fe5a4fa95f9a500f0adc894366a = private unnamed_addr constant [13 x i8] c"SYMBOLIC_LINK", align 1
@alloc_3b304d6af034541cbe6c1768e0d7e144 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00J\02\00\00\1B\00\00\00" }>, align 4
@alloc_2881e96a1d6c2134e67cfef9638c5e77 = private unnamed_addr constant [105 x i8] c"The type of the file descriptor or file is unknown or is different from any of the other types specified.", align 1
@alloc_afa6319c5229c6b9c6d42c58cbe4ad8e = private unnamed_addr constant [59 x i8] c"The file descriptor or file refers to a block device inode.", align 1
@alloc_e6bbd4acfa578c551423adc2eadde5ff = private unnamed_addr constant [63 x i8] c"The file descriptor or file refers to a character device inode.", align 1
@alloc_2eabc60a4165c9f7a9418151f63bad00 = private unnamed_addr constant [56 x i8] c"The file descriptor or file refers to a directory inode.", align 1
@alloc_85306140420f21856a39130a15774ba4 = private unnamed_addr constant [59 x i8] c"The file descriptor or file refers to a regular file inode.", align 1
@alloc_5e9de2c7f264779b97c469a088a5aff8 = private unnamed_addr constant [56 x i8] c"The file descriptor or file refers to a datagram socket.", align 1
@alloc_8a52f02783a958636b88c874240e7bba = private unnamed_addr constant [59 x i8] c"The file descriptor or file refers to a byte-stream socket.", align 1
@alloc_10be943b3d59ff9aac6f8e41e2c46891 = private unnamed_addr constant [41 x i8] c"The file refers to a symbolic link inode.", align 1
@alloc_6c0a2912748dcff49d64307d3d52e0a7 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00N\02\00\00W\02\00\00" }>, align 4
@alloc_5c468b563f3b3351361a94ed4c508401 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\DA\05\00\00\11\00\00\00" }>, align 4
@alloc_56ddcb5ee63e0bd3d6bfaf3d90916595 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\A7\06\00\00\11\00\00\00" }>, align 4
@alloc_86d080ba8845c5fb85fac7d69d229032 = private unnamed_addr constant [5 x i8] c"CLOCK", align 1
@alloc_0bc353ee913f8c550529f784851f4aad = private unnamed_addr constant [7 x i8] c"FD_READ", align 1
@alloc_66d727ea1514b68e4733f24afeaa8c0c = private unnamed_addr constant [8 x i8] c"FD_WRITE", align 1
@alloc_464028a4d23535f5cd5be337a01b3d9f = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\F4\02\00\00\1B\00\00\00" }>, align 4
@alloc_de72e5c289b10c05a483a1229e0c86ca = private unnamed_addr constant [101 x i8] c"The time value of clock `subscription_clock::id` has\0Areached timestamp `subscription_clock::timeout`.", align 1
@alloc_3273b5a611ac24128db1660de333aacf = private unnamed_addr constant [138 x i8] c"File descriptor `subscription_fd_readwrite::file_descriptor` has data\0Aavailable for reading. This event always triggers for regular files.", align 1
@alloc_e5c8cb2c1eabdb1bd09aac14328fb50f = private unnamed_addr constant [142 x i8] c"File descriptor `subscription_fd_readwrite::file_descriptor` has capacity\0Aavailable for writing. This event always triggers for regular files.", align 1
@alloc_e3313cc9a99b89770b9f98d6811105bb = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\05\03\00\00\1B\00\00\00" }>, align 4
@alloc_104041ffccf6d4e22976d0f9e6abad2d = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00\0F\06\00\00\11\00\00\00" }>, align 4
@alloc_5cc1963d30ab6f24862bf937c52293bd = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00G\07\00\00\11\00\00\00" }>, align 4
@alloc_d1d574f28c4f5458a7bfb16314c7c105 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00A\08\00\00\0D\00\00\00" }>, align 4
@alloc_adde0fd5d4a3a7ff0a32f004052d93e0 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00B\08\00\00\0D\00\00\00" }>, align 4
@alloc_727bdb16db3c13deb29d24e6ce1d0490 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_86a8c10f7fc85c93d1bc5807d3c512ac, [12 x i8] c"y\00\00\00b\08\00\00\11\00\00\00" }>, align 4
@alloc_87c86a8fda32926a6ab8441e110c5b98 = private unnamed_addr constant [5 x i8] c"Errno", align 1
@alloc_905976595ed1b08e57e2b44a2acadea4 = private unnamed_addr constant [4 x i8] c"code", align 1
@vtable.0 = private constant <{ [12 x i8], ptr }> <{ [12 x i8] c"\00\00\00\00\02\00\00\00\02\00\00\00", ptr @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u16$GT$3fmt17hc63df5a2f7a3da02E" }>, align 4, !dbg !0
@alloc_f00db71d77c58f05d86c38101680e143 = private unnamed_addr constant [4 x i8] c"name", align 1
@vtable.1 = private constant <{ [12 x i8], ptr }> <{ [12 x i8] c"\00\00\00\00\08\00\00\00\04\00\00\00", ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbffdc5d01bf5408bE" }>, align 4, !dbg !14
@alloc_96af468510ea8f5f9cb1c5ccd138c101 = private unnamed_addr constant [7 x i8] c"message", align 1
@alloc_969ebbff1873a839222fe5aabe10ba14 = private unnamed_addr constant [6 x i8] c"Advice", align 1
@vtable.2 = private constant <{ [12 x i8], ptr }> <{ [12 x i8] c"\00\00\00\00\01\00\00\00\01\00\00\00", ptr @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h923b7f36013e4d7dE" }>, align 4, !dbg !28
@alloc_dc3191724e59b325c68c4dafbe9e2a7b = private unnamed_addr constant [6 x i8] c"Signal", align 1
@alloc_e6d527c7a091e34b073b090f1620dcda = private unnamed_addr constant [6 x i8] c"Whence", align 1
@alloc_fd43fcb84d193089e2ff9355db271b4d = private unnamed_addr constant [7 x i8] c"Clockid", align 1
@vtable.3 = private constant <{ [12 x i8], ptr }> <{ [12 x i8] c"\00\00\00\00\04\00\00\00\04\00\00\00", ptr @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17hfbd5c8efd222f376E" }>, align 4, !dbg !36
@alloc_b48027cc71acf383ef6ed84a39d0e864 = private unnamed_addr constant [8 x i8] c" (error ", align 1
@alloc_9e3f62b0e6490cc45676dc85f910c2d0 = private unnamed_addr constant [1 x i8] c")", align 1
@alloc_c6c92eab63b644b2c14141e96344f22f = private unnamed_addr constant <{ ptr, [4 x i8], ptr, [4 x i8], ptr, [4 x i8] }> <{ ptr inttoptr (i32 1 to ptr), [4 x i8] zeroinitializer, ptr @alloc_b48027cc71acf383ef6ed84a39d0e864, [4 x i8] c"\08\00\00\00", ptr @alloc_9e3f62b0e6490cc45676dc85f910c2d0, [4 x i8] c"\01\00\00\00" }>, align 4
@alloc_19726b29c768e359727fb4780e161989 = private unnamed_addr constant [8 x i8] c"Filetype", align 1
@alloc_56edaa5d846bdfc37ea25102366e3bbb = private unnamed_addr constant [9 x i8] c"Eventtype", align 1
@alloc_227530437c014becb32b0bafc4302ba5 = private unnamed_addr constant [11 x i8] c"Preopentype", align 1

; core::fmt::num::<impl core::fmt::Debug for u8>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h923b7f36013e4d7dE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #0 !dbg !51 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !99, !DIExpression(), !101)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !100, !DIExpression(), !102)
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4 %f) #52, !dbg !103
  br i1 %_3, label %bb2, label %bb3, !dbg !104

bb3:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_5 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4 %f) #52, !dbg !105
  br i1 %_5, label %bb5, label %bb6, !dbg !106

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for u8>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hf19d67508dd58d4aE"(ptr align 1 %self, ptr align 4 %f) #52, !dbg !107
  %1 = zext i1 %0 to i8, !dbg !107
  store i8 %1, ptr %_0, align 1, !dbg !107
  br label %bb7, !dbg !107

bb6:                                              ; preds = %bb3
; call core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt
  %2 = call zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf2721191f040b59aE"(ptr align 1 %self, ptr align 4 %f) #52, !dbg !108
  %3 = zext i1 %2 to i8, !dbg !108
  store i8 %3, ptr %_0, align 1, !dbg !108
  br label %bb7, !dbg !108

bb5:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::UpperHex for u8>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h3a28df7a448ec4e8E"(ptr align 1 %self, ptr align 4 %f) #52, !dbg !109
  %5 = zext i1 %4 to i8, !dbg !109
  store i8 %5, ptr %_0, align 1, !dbg !109
  br label %bb7, !dbg !109

bb7:                                              ; preds = %bb2, %bb5, %bb6
  %6 = load i8, ptr %_0, align 1, !dbg !110
  %7 = trunc nuw i8 %6 to i1, !dbg !110
  ret i1 %7, !dbg !110
}

; core::fmt::num::<impl core::fmt::Debug for u16>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u16$GT$3fmt17hc63df5a2f7a3da02E"(ptr align 2 %self, ptr align 4 %f) unnamed_addr #0 !dbg !111 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !117, !DIExpression(), !119)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !118, !DIExpression(), !120)
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4 %f) #52, !dbg !121
  br i1 %_3, label %bb2, label %bb3, !dbg !122

bb3:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_5 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4 %f) #52, !dbg !123
  br i1 %_5, label %bb5, label %bb6, !dbg !124

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for u16>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17hfc6eebde7cab5daeE"(ptr align 2 %self, ptr align 4 %f) #52, !dbg !125
  %1 = zext i1 %0 to i8, !dbg !125
  store i8 %1, ptr %_0, align 1, !dbg !125
  br label %bb7, !dbg !125

bb6:                                              ; preds = %bb3
; call core::fmt::num::imp::<impl core::fmt::Display for u16>::fmt
  %2 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u16$GT$3fmt17hbe9cdadc62270c33E"(ptr align 2 %self, ptr align 4 %f) #52, !dbg !126
  %3 = zext i1 %2 to i8, !dbg !126
  store i8 %3, ptr %_0, align 1, !dbg !126
  br label %bb7, !dbg !126

bb5:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::UpperHex for u16>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17hf846c66d16667c7bE"(ptr align 2 %self, ptr align 4 %f) #52, !dbg !127
  %5 = zext i1 %4 to i8, !dbg !127
  store i8 %5, ptr %_0, align 1, !dbg !127
  br label %bb7, !dbg !127

bb7:                                              ; preds = %bb2, %bb5, %bb6
  %6 = load i8, ptr %_0, align 1, !dbg !128
  %7 = trunc nuw i8 %6 to i1, !dbg !128
  ret i1 %7, !dbg !128
}

; core::fmt::num::<impl core::fmt::Debug for u32>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17hfbd5c8efd222f376E"(ptr align 4 %self, ptr align 4 %f) unnamed_addr #0 !dbg !129 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !135, !DIExpression(), !137)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !136, !DIExpression(), !138)
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4 %f) #52, !dbg !139
  br i1 %_3, label %bb2, label %bb3, !dbg !140

bb3:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_5 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4 %f) #52, !dbg !141
  br i1 %_5, label %bb5, label %bb6, !dbg !142

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for u32>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17hc2f3e52917b39c8dE"(ptr align 4 %self, ptr align 4 %f) #52, !dbg !143
  %1 = zext i1 %0 to i8, !dbg !143
  store i8 %1, ptr %_0, align 1, !dbg !143
  br label %bb7, !dbg !143

bb6:                                              ; preds = %bb3
; call core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
  %2 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hd4bf219ffc62a5f7E"(ptr align 4 %self, ptr align 4 %f) #52, !dbg !144
  %3 = zext i1 %2 to i8, !dbg !144
  store i8 %3, ptr %_0, align 1, !dbg !144
  br label %bb7, !dbg !144

bb5:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::UpperHex for u32>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17he005099c42d93007E"(ptr align 4 %self, ptr align 4 %f) #52, !dbg !145
  %5 = zext i1 %4 to i8, !dbg !145
  store i8 %5, ptr %_0, align 1, !dbg !145
  br label %bb7, !dbg !145

bb7:                                              ; preds = %bb2, %bb5, %bb6
  %6 = load i8, ptr %_0, align 1, !dbg !146
  %7 = trunc nuw i8 %6 to i1, !dbg !146
  ret i1 %7, !dbg !146
}

; core::fmt::Arguments::as_statically_known_str
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @_ZN4core3fmt9Arguments23as_statically_known_str17h4ba4e91277018bdbE(ptr align 4 %self) unnamed_addr #0 !dbg !147 {
start:
  %0 = alloca [1 x i8], align 1
  %self.dbg.spill = alloca [4 x i8], align 4
  %s = alloca [8 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !255, !DIExpression(), !258)
    #dbg_declare(ptr %s, !256, !DIExpression(), !259)
; call core::fmt::Arguments::as_str
  %1 = call { ptr, i32 } @_ZN4core3fmt9Arguments6as_str17h5f43f546cd1c7996E(ptr align 4 %self) #52, !dbg !260
  %2 = extractvalue { ptr, i32 } %1, 0, !dbg !260
  %3 = extractvalue { ptr, i32 } %1, 1, !dbg !260
  store ptr %2, ptr %s, align 4, !dbg !260
  %4 = getelementptr inbounds i8, ptr %s, i32 4, !dbg !260
  store i32 %3, ptr %4, align 4, !dbg !260
; call core::option::Option<T>::is_some
  %_4 = call zeroext i1 @"_ZN4core6option15Option$LT$T$GT$7is_some17h59be6c2f34b18cc5E"(ptr align 4 %s) #52, !dbg !261
  %5 = call i1 @llvm.is.constant.i1(i1 %_4), !dbg !262
  %6 = zext i1 %5 to i8, !dbg !262
  store i8 %6, ptr %0, align 1, !dbg !262
  %7 = load i8, ptr %0, align 1, !dbg !262
  %_3 = trunc nuw i8 %7 to i1, !dbg !262
  br i1 %_3, label %bb4, label %bb5, !dbg !262

bb5:                                              ; preds = %start
  store ptr null, ptr %_0, align 4, !dbg !263
  br label %bb6, !dbg !264

bb4:                                              ; preds = %start
  %8 = load ptr, ptr %s, align 4, !dbg !265
  %9 = getelementptr inbounds i8, ptr %s, i32 4, !dbg !265
  %10 = load i32, ptr %9, align 4, !dbg !265
  store ptr %8, ptr %_0, align 4, !dbg !265
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !265
  store i32 %10, ptr %11, align 4, !dbg !265
  br label %bb6, !dbg !264

bb6:                                              ; preds = %bb4, %bb5
  %12 = load ptr, ptr %_0, align 4, !dbg !266
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !266
  %14 = load i32, ptr %13, align 4, !dbg !266
  %15 = insertvalue { ptr, i32 } poison, ptr %12, 0, !dbg !266
  %16 = insertvalue { ptr, i32 } %15, i32 %14, 1, !dbg !266
  ret { ptr, i32 } %16, !dbg !266
}

; core::fmt::Arguments::as_str
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @_ZN4core3fmt9Arguments6as_str17h5f43f546cd1c7996E(ptr align 4 %self) unnamed_addr #0 !dbg !267 {
start:
  %s.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_2 = alloca [16 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !270, !DIExpression(), !274)
  %_3.0 = load ptr, ptr %self, align 4, !dbg !275
  %0 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !275
  %_3.1 = load i32, ptr %0, align 4, !dbg !275
  %1 = getelementptr inbounds i8, ptr %self, i32 8, !dbg !276
  %_4.0 = load ptr, ptr %1, align 4, !dbg !276
  %2 = getelementptr inbounds i8, ptr %1, i32 4, !dbg !276
  %_4.1 = load i32, ptr %2, align 4, !dbg !276
  store ptr %_3.0, ptr %_2, align 4, !dbg !277
  %3 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !277
  store i32 %_3.1, ptr %3, align 4, !dbg !277
  %4 = getelementptr inbounds i8, ptr %_2, i32 8, !dbg !277
  store ptr %_4.0, ptr %4, align 4, !dbg !277
  %5 = getelementptr inbounds i8, ptr %4, i32 4, !dbg !277
  store i32 %_4.1, ptr %5, align 4, !dbg !277
  %_15.0 = load ptr, ptr %_2, align 4, !dbg !278
  %6 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !278
  %_15.1 = load i32, ptr %6, align 4, !dbg !278
  %7 = icmp eq i32 %_15.1, 0, !dbg !278
  br i1 %7, label %bb2, label %bb3, !dbg !278

bb2:                                              ; preds = %start
  %8 = getelementptr inbounds i8, ptr %_2, i32 8, !dbg !279
  %_16.0 = load ptr, ptr %8, align 4, !dbg !279
  %9 = getelementptr inbounds i8, ptr %8, i32 4, !dbg !279
  %_16.1 = load i32, ptr %9, align 4, !dbg !279
  %10 = icmp eq i32 %_16.1, 0, !dbg !279
  br i1 %10, label %bb6, label %bb1, !dbg !279

bb3:                                              ; preds = %start
  %_17.0 = load ptr, ptr %_2, align 4, !dbg !280
  %11 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !280
  %_17.1 = load i32, ptr %11, align 4, !dbg !280
  %12 = icmp eq i32 %_17.1, 1, !dbg !280
  br i1 %12, label %bb4, label %bb1, !dbg !280

bb6:                                              ; preds = %bb2
  store ptr inttoptr (i32 1 to ptr), ptr %_0, align 4, !dbg !281
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !281
  store i32 0, ptr %13, align 4, !dbg !281
  br label %bb7, !dbg !282

bb1:                                              ; preds = %bb4, %bb3, %bb2
  store ptr null, ptr %_0, align 4, !dbg !283
  br label %bb7, !dbg !283

bb7:                                              ; preds = %bb1, %bb5, %bb6
  %14 = load ptr, ptr %_0, align 4, !dbg !284
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !284
  %16 = load i32, ptr %15, align 4, !dbg !284
  %17 = insertvalue { ptr, i32 } poison, ptr %14, 0, !dbg !284
  %18 = insertvalue { ptr, i32 } %17, i32 %16, 1, !dbg !284
  ret { ptr, i32 } %18, !dbg !284

bb4:                                              ; preds = %bb3
  %19 = getelementptr inbounds i8, ptr %_2, i32 8, !dbg !285
  %_18.0 = load ptr, ptr %19, align 4, !dbg !285
  %20 = getelementptr inbounds i8, ptr %19, i32 4, !dbg !285
  %_18.1 = load i32, ptr %20, align 4, !dbg !285
  %21 = icmp eq i32 %_18.1, 0, !dbg !285
  br i1 %21, label %bb5, label %bb1, !dbg !285

bb5:                                              ; preds = %bb4
  %_19.0 = load ptr, ptr %_2, align 4, !dbg !286
  %22 = getelementptr inbounds i8, ptr %_2, i32 4, !dbg !286
  %_19.1 = load i32, ptr %22, align 4, !dbg !286
  %s = getelementptr inbounds nuw { ptr, i32 }, ptr %_19.0, i32 0, !dbg !286
  store ptr %s, ptr %s.dbg.spill, align 4, !dbg !286
    #dbg_declare(ptr %s.dbg.spill, !271, !DIExpression(), !287)
  %_20.0 = load ptr, ptr %s, align 4, !dbg !288
  %23 = getelementptr inbounds i8, ptr %s, i32 4, !dbg !288
  %_20.1 = load i32, ptr %23, align 4, !dbg !288
  store ptr %_20.0, ptr %_0, align 4, !dbg !289
  %24 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !289
  store i32 %_20.1, ptr %24, align 4, !dbg !289
  br label %bb7, !dbg !290
}

; core::fmt::Formatter::write_fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core3fmt9Formatter9write_fmt17h5e1a4779fbbec593E(ptr align 4 %self, ptr align 4 %fmt) unnamed_addr #0 !dbg !291 {
start:
  %0 = alloca [24 x i8], align 4
  %s.dbg.spill = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_3 = alloca [8 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !296, !DIExpression(), !300)
    #dbg_declare(ptr %fmt, !297, !DIExpression(), !301)
; call core::fmt::Arguments::as_statically_known_str
  %1 = call { ptr, i32 } @_ZN4core3fmt9Arguments23as_statically_known_str17h4ba4e91277018bdbE(ptr align 4 %fmt) #52, !dbg !302
  %2 = extractvalue { ptr, i32 } %1, 0, !dbg !302
  %3 = extractvalue { ptr, i32 } %1, 1, !dbg !302
  store ptr %2, ptr %_3, align 4, !dbg !302
  %4 = getelementptr inbounds i8, ptr %_3, i32 4, !dbg !302
  store i32 %3, ptr %4, align 4, !dbg !302
  %5 = load ptr, ptr %_3, align 4, !dbg !303
  %6 = getelementptr inbounds i8, ptr %_3, i32 4, !dbg !303
  %7 = load i32, ptr %6, align 4, !dbg !303
  %8 = ptrtoint ptr %5 to i32, !dbg !303
  %9 = icmp eq i32 %8, 0, !dbg !303
  %_5 = select i1 %9, i32 0, i32 1, !dbg !303
  %10 = trunc nuw i32 %_5 to i1, !dbg !304
  br i1 %10, label %bb2, label %bb3, !dbg !304

bb2:                                              ; preds = %start
  %s.0 = load ptr, ptr %_3, align 4, !dbg !305
  %11 = getelementptr inbounds i8, ptr %_3, i32 4, !dbg !305
  %s.1 = load i32, ptr %11, align 4, !dbg !305
  store ptr %s.0, ptr %s.dbg.spill, align 4, !dbg !305
  %12 = getelementptr inbounds i8, ptr %s.dbg.spill, i32 4, !dbg !305
  store i32 %s.1, ptr %12, align 4, !dbg !305
    #dbg_declare(ptr %s.dbg.spill, !298, !DIExpression(), !305)
  %_7.0 = load ptr, ptr %self, align 4, !dbg !306
  %13 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !306
  %_7.1 = load ptr, ptr %13, align 4, !dbg !306
  %14 = getelementptr inbounds i8, ptr %_7.1, i32 12, !dbg !306
  %15 = load ptr, ptr %14, align 4, !dbg !306, !invariant.load !13, !nonnull !13
  %16 = call zeroext i1 %15(ptr align 1 %_7.0, ptr align 1 %s.0, i32 %s.1) #52, !dbg !307
  %17 = zext i1 %16 to i8, !dbg !307
  store i8 %17, ptr %_0, align 1, !dbg !307
  br label %bb4, !dbg !307

bb3:                                              ; preds = %start
  %_8.0 = load ptr, ptr %self, align 4, !dbg !308
  %18 = getelementptr inbounds i8, ptr %self, i32 4, !dbg !308
  %_8.1 = load ptr, ptr %18, align 4, !dbg !308
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %0, ptr align 4 %fmt, i32 24, i1 false), !dbg !309
; call core::fmt::write
  %19 = call zeroext i1 @_ZN4core3fmt5write17hc2899684fc6bf93bE(ptr align 1 %_8.0, ptr align 4 %_8.1, ptr align 4 %0) #52, !dbg !309
  %20 = zext i1 %19 to i8, !dbg !309
  store i8 %20, ptr %_0, align 1, !dbg !309
  br label %bb4, !dbg !309

bb4:                                              ; preds = %bb3, %bb2
  %21 = load i8, ptr %_0, align 1, !dbg !310
  %22 = trunc nuw i8 %21 to i1, !dbg !310
  ret i1 %22, !dbg !310

bb5:                                              ; No predecessors!
  unreachable, !dbg !311
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local i16 @_ZN4core3ptr4read17h188bb76cc8094201E(ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !312 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !337, !DIExpression(), !340)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !341
  br i1 %_2, label %bb2, label %bb4, !dbg !341

bb4:                                              ; preds = %bb2, %start
  %_0 = load i16, ptr %src, align 2, !dbg !344
  ret i16 %_0, !dbg !345

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 2, i1 zeroext false, ptr align 4 %0) #52, !dbg !346
  br label %bb4, !dbg !346
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local void @_ZN4core3ptr4read17h28b571f9788304ccE(ptr sret([24 x i8]) align 8 %_0, ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !347 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !364, !DIExpression(), !367)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !368
  br i1 %_2, label %bb2, label %bb4, !dbg !368

bb4:                                              ; preds = %bb2, %start
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %_0, ptr align 8 %src, i32 24, i1 false), !dbg !370
  ret void, !dbg !371

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 8, i1 zeroext false, ptr align 4 %0) #52, !dbg !372
  br label %bb4, !dbg !372
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local i64 @_ZN4core3ptr4read17h487dc6145fad69b1E(ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !373 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !378, !DIExpression(), !381)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !382
  br i1 %_2, label %bb2, label %bb4, !dbg !382

bb4:                                              ; preds = %bb2, %start
  %_0 = load i64, ptr %src, align 8, !dbg !384
  ret i64 %_0, !dbg !385

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 8, i1 zeroext false, ptr align 4 %0) #52, !dbg !386
  br label %bb4, !dbg !386
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local void @_ZN4core3ptr4read17h58671998979cdf8bE(ptr sret([8 x i8]) align 4 %_0, ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !387 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !402, !DIExpression(), !405)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !406
  br i1 %_2, label %bb2, label %bb4, !dbg !406

bb4:                                              ; preds = %bb2, %start
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_0, ptr align 4 %src, i32 8, i1 false), !dbg !408
  ret void, !dbg !409

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 4, i1 zeroext false, ptr align 4 %0) #52, !dbg !410
  br label %bb4, !dbg !410
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local void @_ZN4core3ptr4read17hc08313af9d479144E(ptr sret([64 x i8]) align 8 %_0, ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !411 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !426, !DIExpression(), !429)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !430
  br i1 %_2, label %bb2, label %bb4, !dbg !430

bb4:                                              ; preds = %bb2, %start
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %_0, ptr align 8 %src, i32 64, i1 false), !dbg !432
  ret void, !dbg !433

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 8, i1 zeroext false, ptr align 4 %0) #52, !dbg !434
  br label %bb4, !dbg !434
}

; core::ptr::read
; Function Attrs: inlinehint nounwind
define dso_local i32 @_ZN4core3ptr4read17he4d71e30ba8af448E(ptr %src, ptr align 4 %0) unnamed_addr #0 !dbg !435 {
start:
  %src.dbg.spill = alloca [4 x i8], align 4
  store ptr %src, ptr %src.dbg.spill, align 4
    #dbg_declare(ptr %src.dbg.spill, !440, !DIExpression(), !443)
; call core::ub_checks::check_language_ub
  %_2 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !444
  br i1 %_2, label %bb2, label %bb4, !dbg !444

bb4:                                              ; preds = %bb2, %start
  %_0 = load i32, ptr %src, align 4, !dbg !446
  ret i32 %_0, !dbg !447

bb2:                                              ; preds = %start
; call core::ptr::read::precondition_check
  call void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %src, i32 4, i1 zeroext false, ptr align 4 %0) #52, !dbg !448
  br label %bb4, !dbg !448
}

; core::ptr::read::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE(ptr %addr, i32 %align, i1 zeroext %is_zst, ptr align 4 %0) unnamed_addr #0 !dbg !449 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %addr.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [8 x i8], align 4
  %_6 = alloca [24 x i8], align 4
  store ptr %addr, ptr %addr.dbg.spill, align 4
    #dbg_declare(ptr %addr.dbg.spill, !455, !DIExpression(), !460)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !456, !DIExpression(), !460)
  %1 = zext i1 %is_zst to i8
  store i8 %1, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !457, !DIExpression(), !460)
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %msg.dbg.spill, align 4, !dbg !461
  %2 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !461
  store i32 218, ptr %2, align 4, !dbg !461
    #dbg_declare(ptr %msg.dbg.spill, !458, !DIExpression(), !461)
; call core::ub_checks::maybe_is_aligned_and_not_null
  %_4 = call zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hc491a94493deec50E(ptr %addr, i32 %align, i1 zeroext %is_zst) #52, !dbg !462
  br i1 %_4, label %bb2, label %bb3, !dbg !462

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds nuw { ptr, i32 }, ptr %_8, i32 0, !dbg !464
  store ptr @alloc_ed8641ebea8e5515740d4eb49a916ff5, ptr %3, align 4, !dbg !464
  %4 = getelementptr inbounds i8, ptr %3, i32 4, !dbg !464
  store i32 218, ptr %4, align 4, !dbg !464
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_6, ptr align 4 %_8) #52, !dbg !465
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h898da05acec7acadE(ptr align 4 %_6, i1 zeroext false, ptr align 4 %0) #53, !dbg !466
  unreachable, !dbg !466

bb2:                                              ; preds = %start
  ret void, !dbg !467
}

; core::str::<impl str>::len
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %self.0, i32 %self.1) unnamed_addr #0 !dbg !468 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %self.dbg.spill = alloca [8 x i8], align 4
  store ptr %self.0, ptr %self.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %self.dbg.spill, i32 4
  store i32 %self.1, ptr %0, align 4
    #dbg_declare(ptr %self.dbg.spill, !475, !DIExpression(), !476)
  store ptr %self.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %self.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !477, !DIExpression(), !486)
  %2 = insertvalue { ptr, i32 } poison, ptr %self.0, 0, !dbg !488
  %3 = insertvalue { ptr, i32 } %2, i32 %self.1, 1, !dbg !488
  %_2.0 = extractvalue { ptr, i32 } %3, 0, !dbg !489
  %_2.1 = extractvalue { ptr, i32 } %3, 1, !dbg !489
  ret i32 %_2.1, !dbg !490
}

; core::hint::unreachable_unchecked
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 %0) unnamed_addr #1 !dbg !491 {
start:
; call core::ub_checks::check_language_ub
  %_1 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() #52, !dbg !496
  br i1 %_1, label %bb2, label %bb3, !dbg !496

bb3:                                              ; preds = %bb2, %start
  unreachable, !dbg !498

bb2:                                              ; preds = %start
; call core::hint::unreachable_unchecked::precondition_check
  call void @_ZN4core4hint21unreachable_unchecked18precondition_check17ha0410ca2683f1fabE(ptr align 4 %0) #52, !dbg !499
  br label %bb3, !dbg !499
}

; core::hint::unreachable_unchecked::precondition_check
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core4hint21unreachable_unchecked18precondition_check17ha0410ca2683f1fabE(ptr align 4 %0) unnamed_addr #0 !dbg !500 {
start:
  %msg.dbg.spill = alloca [8 x i8], align 4
  %_4 = alloca [8 x i8], align 4
  %_2 = alloca [24 x i8], align 4
  store ptr @alloc_75fb06c2453febd814e73f5f2e72ae38, ptr %msg.dbg.spill, align 4, !dbg !505
  %1 = getelementptr inbounds i8, ptr %msg.dbg.spill, i32 4, !dbg !505
  store i32 199, ptr %1, align 4, !dbg !505
    #dbg_declare(ptr %msg.dbg.spill, !503, !DIExpression(), !505)
  %2 = getelementptr inbounds nuw { ptr, i32 }, ptr %_4, i32 0, !dbg !506
  store ptr @alloc_75fb06c2453febd814e73f5f2e72ae38, ptr %2, align 4, !dbg !506
  %3 = getelementptr inbounds i8, ptr %2, i32 4, !dbg !506
  store i32 199, ptr %3, align 4, !dbg !506
; call core::fmt::rt::<impl core::fmt::Arguments>::new_const
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4 %_2, ptr align 4 %_4) #52, !dbg !507
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17h898da05acec7acadE(ptr align 4 %_2, i1 zeroext false, ptr align 4 %0) #53, !dbg !508
  unreachable, !dbg !508
}

; core::panicking::panic_nounwind_fmt
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt17h898da05acec7acadE(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #1 !dbg !509 {
start:
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %_3 = alloca [28 x i8], align 4
    #dbg_declare(ptr %fmt, !515, !DIExpression(), !517)
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !516, !DIExpression(), !518)
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_3, ptr align 4 %fmt, i32 24, i1 false), !dbg !519
  %2 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !519
  %3 = zext i1 %force_no_backtrace to i8, !dbg !519
  store i8 %3, ptr %2, align 4, !dbg !519
  %4 = getelementptr inbounds i8, ptr %_3, i32 24, !dbg !522
  %5 = load i8, ptr %4, align 4, !dbg !522
  %6 = trunc nuw i8 %5 to i1, !dbg !522
; call core::panicking::panic_nounwind_fmt::runtime
  call void @_ZN4core9panicking18panic_nounwind_fmt7runtime17h0317e615256657b0E(ptr align 4 %_3, i1 zeroext %6, ptr align 4 %0) #53, !dbg !522
  unreachable, !dbg !522
}

; core::panicking::panic_nounwind_fmt::runtime
; Function Attrs: inlinehint noreturn nounwind
define internal void @_ZN4core9panicking18panic_nounwind_fmt7runtime17h0317e615256657b0E(ptr align 4 %fmt, i1 zeroext %force_no_backtrace, ptr align 4 %0) unnamed_addr #1 !dbg !523 {
start:
    #dbg_declare(ptr %fmt, !526, !DIExpression(), !537)
  %force_no_backtrace.dbg.spill = alloca [1 x i8], align 1
  %1 = zext i1 %force_no_backtrace to i8
  store i8 %1, ptr %force_no_backtrace.dbg.spill, align 1
    #dbg_declare(ptr %force_no_backtrace.dbg.spill, !527, !DIExpression(), !537)
  call void @llvm.trap(), !dbg !538
  unreachable, !dbg !538
}

; core::ub_checks::maybe_is_aligned
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17haad94b0cc6d1077dE(ptr %ptr, i32 %align) unnamed_addr #0 !dbg !540 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !545, !DIExpression(), !547)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !546, !DIExpression(), !548)
; call core::ub_checks::maybe_is_aligned::runtime
  %_0 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17hda16d0ba0b9a56bdE(ptr %ptr, i32 %align) #52, !dbg !549
  ret i1 %_0, !dbg !551
}

; core::ub_checks::maybe_is_aligned::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned7runtime17hda16d0ba0b9a56bdE(ptr %ptr, i32 %align) unnamed_addr #0 !dbg !552 {
start:
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !555, !DIExpression(), !557)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !556, !DIExpression(), !557)
; call core::ptr::const_ptr::<impl *const T>::is_aligned_to
  %_0 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr %ptr, i32 %align) #52, !dbg !558
  ret i1 %_0, !dbg !560
}

; core::ub_checks::check_language_ub
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE() unnamed_addr #0 !dbg !561 {
start:
  %_0 = alloca [1 x i8], align 1
  br label %bb1, !dbg !564

bb1:                                              ; preds = %start
; call core::ub_checks::check_language_ub::runtime
  %0 = call zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h0715ca3a72765d67E() #52, !dbg !565
  %1 = zext i1 %0 to i8, !dbg !565
  store i8 %1, ptr %_0, align 1, !dbg !565
  br label %bb3, !dbg !565

bb3:                                              ; preds = %bb1
  %2 = load i8, ptr %_0, align 1, !dbg !567
  %3 = trunc nuw i8 %2 to i1, !dbg !567
  ret i1 %3, !dbg !567

bb2:                                              ; No predecessors!
  unreachable
}

; core::ub_checks::check_language_ub::runtime
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks17check_language_ub7runtime17h0715ca3a72765d67E() unnamed_addr #0 !dbg !568 {
start:
  ret i1 true, !dbg !570
}

; core::ub_checks::maybe_is_aligned_and_not_null
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hc491a94493deec50E(ptr %ptr, i32 %align, i1 zeroext %is_zst) unnamed_addr #0 !dbg !571 {
start:
  %is_zst.dbg.spill = alloca [1 x i8], align 1
  %align.dbg.spill = alloca [4 x i8], align 4
  %ptr.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [1 x i8], align 1
  store ptr %ptr, ptr %ptr.dbg.spill, align 4
    #dbg_declare(ptr %ptr.dbg.spill, !575, !DIExpression(), !578)
  store i32 %align, ptr %align.dbg.spill, align 4
    #dbg_declare(ptr %align.dbg.spill, !576, !DIExpression(), !579)
  %0 = zext i1 %is_zst to i8
  store i8 %0, ptr %is_zst.dbg.spill, align 1
    #dbg_declare(ptr %is_zst.dbg.spill, !577, !DIExpression(), !580)
; call core::ub_checks::maybe_is_aligned
  %_4 = call zeroext i1 @_ZN4core9ub_checks16maybe_is_aligned17haad94b0cc6d1077dE(ptr %ptr, i32 %align) #52, !dbg !581
  br i1 %_4, label %bb2, label %bb3, !dbg !581

bb3:                                              ; preds = %start
  store i8 0, ptr %_0, align 1, !dbg !581
  br label %bb7, !dbg !581

bb2:                                              ; preds = %start
  br i1 %is_zst, label %bb4, label %bb5, !dbg !582

bb7:                                              ; preds = %bb4, %bb5, %bb3
  %1 = load i8, ptr %_0, align 1, !dbg !583
  %2 = trunc nuw i8 %1 to i1, !dbg !583
  ret i1 %2, !dbg !583

bb5:                                              ; preds = %bb2
; call core::ptr::const_ptr::<impl *const T>::is_null
  %_5 = call zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr %ptr) #52, !dbg !584
  %3 = xor i1 %_5, true, !dbg !585
  %4 = zext i1 %3 to i8, !dbg !585
  store i8 %4, ptr %_0, align 1, !dbg !585
  br label %bb7, !dbg !586

bb4:                                              ; preds = %bb2
  store i8 1, ptr %_0, align 1, !dbg !586
  br label %bb7, !dbg !586
}

; wasi::lib_generated::fd_readdir
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated10fd_readdir17h70dd87b0fd008942E(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr %buf, i32 %buf_len, i64 %cookie) unnamed_addr #2 !dbg !587 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %cookie.dbg.spill = alloca [8 x i8], align 8
  %buf_len.dbg.spill = alloca [4 x i8], align 4
  %buf.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !612, !DIExpression(), !632)
  store ptr %buf, ptr %buf.dbg.spill, align 4
    #dbg_declare(ptr %buf.dbg.spill, !613, !DIExpression(), !633)
  store i32 %buf_len, ptr %buf_len.dbg.spill, align 4
    #dbg_declare(ptr %buf_len.dbg.spill, !614, !DIExpression(), !634)
  store i64 %cookie, ptr %cookie.dbg.spill, align 8
    #dbg_declare(ptr %cookie.dbg.spill, !615, !DIExpression(), !635)
    #dbg_declare(ptr %rp0, !616, !DIExpression(), !636)
  store i32 undef, ptr %rp0, align 4, !dbg !637
  %_8 = ptrtoint ptr %buf to i32, !dbg !638
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !648)
  %_11 = ptrtoint ptr %rp0 to i32, !dbg !650
; call wasi::lib_generated::wasi_snapshot_preview1::fd_readdir
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110fd_readdir17hadee90ca69ca3b01E(i32 %fd, i32 %_8, i32 %buf_len, i64 %cookie, i32 %_11) #52, !dbg !651
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !651
    #dbg_declare(ptr %ret.dbg.spill, !629, !DIExpression(), !652)
  %0 = icmp eq i32 %ret, 0, !dbg !653
  br i1 %0, label %bb5, label %bb4, !dbg !653

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !654)
  %_16 = ptrtoint ptr %rp0 to i32, !dbg !656
  %_15 = inttoptr i32 %_16 to ptr, !dbg !656
; call core::ptr::read
  %_14 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_15, ptr align 4 @alloc_67e747fb881d0fb15b51613bd08b794b) #52, !dbg !657
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !658
  store i32 %_14, ptr %1, align 4, !dbg !658
  store i16 0, ptr %_0, align 4, !dbg !658
  br label %bb8, !dbg !659

bb4:                                              ; preds = %start
  %_20 = trunc i32 %ret to i16, !dbg !660
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !661
  store i16 %_20, ptr %2, align 2, !dbg !661
  store i16 1, ptr %_0, align 4, !dbg !661
  br label %bb8, !dbg !662

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !663
}

; wasi::lib_generated::proc_raise
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated10proc_raise17h7a2e0aeb3b0e2c49E(i8 %sig) unnamed_addr #2 !dbg !664 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %sig.dbg.spill = alloca [1 x i8], align 1
  %_0 = alloca [4 x i8], align 2
  store i8 %sig, ptr %sig.dbg.spill, align 1
    #dbg_declare(ptr %sig.dbg.spill, !685, !DIExpression(), !688)
  %_3 = zext i8 %sig to i32, !dbg !689
; call wasi::lib_generated::wasi_snapshot_preview1::proc_raise
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110proc_raise17h88509519ac6671ebE(i32 %_3) #52, !dbg !690
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !690
    #dbg_declare(ptr %ret.dbg.spill, !686, !DIExpression(), !691)
  %0 = icmp eq i32 %ret, 0, !dbg !692
  br i1 %0, label %bb3, label %bb2, !dbg !692

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !693
  br label %bb4, !dbg !694

bb2:                                              ; preds = %start
  %_6 = trunc i32 %ret to i16, !dbg !695
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !696
  store i16 %_6, ptr %1, align 2, !dbg !696
  store i16 1, ptr %_0, align 2, !dbg !696
  br label %bb4, !dbg !697

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !698
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !698
  %4 = load i16, ptr %3, align 2, !dbg !698
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !698
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !698
  ret { i16, i16 } %6, !dbg !698
}

; wasi::lib_generated::random_get
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated10random_get17h623dfe08f2cd86edE(ptr %buf, i32 %buf_len) unnamed_addr #2 !dbg !699 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %buf_len.dbg.spill = alloca [4 x i8], align 4
  %buf.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store ptr %buf, ptr %buf.dbg.spill, align 4
    #dbg_declare(ptr %buf.dbg.spill, !703, !DIExpression(), !707)
  store i32 %buf_len, ptr %buf_len.dbg.spill, align 4
    #dbg_declare(ptr %buf_len.dbg.spill, !704, !DIExpression(), !708)
  %_4 = ptrtoint ptr %buf to i32, !dbg !709
; call wasi::lib_generated::wasi_snapshot_preview1::random_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110random_get17h8b3b06eaef0413dfE(i32 %_4, i32 %buf_len) #52, !dbg !710
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !710
    #dbg_declare(ptr %ret.dbg.spill, !705, !DIExpression(), !711)
  %0 = icmp eq i32 %ret, 0, !dbg !712
  br i1 %0, label %bb3, label %bb2, !dbg !712

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !713
  br label %bb4, !dbg !714

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !715
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !716
  store i16 %_7, ptr %1, align 2, !dbg !716
  store i16 1, ptr %_0, align 2, !dbg !716
  br label %bb4, !dbg !717

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !718
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !718
  %4 = load i16, ptr %3, align 2, !dbg !718
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !718
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !718
  ret { i16, i16 } %6, !dbg !718
}

; wasi::lib_generated::Preopentype::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated11Preopentype3raw17h66bac64334fbea7cE(ptr align 1 %self) unnamed_addr #2 !dbg !719 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !728, !DIExpression(), !729)
  %_0 = load i8, ptr %self, align 1, !dbg !730
  ret i8 %_0, !dbg !731
}

; wasi::lib_generated::Preopentype::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated11Preopentype4name17h51a1ca3954e9478cE(ptr align 1 %self) unnamed_addr #2 !dbg !732 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !737, !DIExpression(), !738)
  %0 = load i8, ptr %self, align 1, !dbg !739
  %1 = icmp eq i8 %0, 0, !dbg !739
  br i1 %1, label %bb2, label %bb1, !dbg !739

bb2:                                              ; preds = %start
  ret { ptr, i32 } { ptr @alloc_c6ac80460b9305d5066a706b621b8266, i32 3 }, !dbg !740

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_f06bcdf8814ea766c9e9862e5f212d96) #53, !dbg !741
  unreachable, !dbg !741
}

; wasi::lib_generated::Preopentype::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated11Preopentype7message17ha683a318ca855ec8E(ptr align 1 %self) unnamed_addr #2 !dbg !742 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !745, !DIExpression(), !746)
  %0 = load i8, ptr %self, align 1, !dbg !747
  %1 = icmp eq i8 %0, 0, !dbg !747
  br i1 %1, label %bb2, label %bb1, !dbg !747

bb2:                                              ; preds = %start
  ret { ptr, i32 } { ptr @alloc_b9865f014425c9240a60a0593d75bfd4, i32 23 }, !dbg !748

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_bf1d48e0a063db09479fdcde4ace0db2) #53, !dbg !749
  unreachable, !dbg !749
}

; wasi::lib_generated::environ_get
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11environ_get17he9729f8c36e4c46eE(ptr %environ, ptr %environ_buf) unnamed_addr #2 !dbg !750 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %environ_buf.dbg.spill = alloca [4 x i8], align 4
  %environ.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store ptr %environ, ptr %environ.dbg.spill, align 4
    #dbg_declare(ptr %environ.dbg.spill, !755, !DIExpression(), !759)
  store ptr %environ_buf, ptr %environ_buf.dbg.spill, align 4
    #dbg_declare(ptr %environ_buf.dbg.spill, !756, !DIExpression(), !760)
  %_4 = ptrtoint ptr %environ to i32, !dbg !761
  %_5 = ptrtoint ptr %environ_buf to i32, !dbg !762
; call wasi::lib_generated::wasi_snapshot_preview1::environ_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111environ_get17h8a9aabfc46a9fff9E(i32 %_4, i32 %_5) #52, !dbg !763
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !763
    #dbg_declare(ptr %ret.dbg.spill, !757, !DIExpression(), !764)
  %0 = icmp eq i32 %ret, 0, !dbg !765
  br i1 %0, label %bb3, label %bb2, !dbg !765

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !766
  br label %bb4, !dbg !767

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !768
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !769
  store i16 %_7, ptr %1, align 2, !dbg !769
  store i16 1, ptr %_0, align 2, !dbg !769
  br label %bb4, !dbg !770

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !771
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !771
  %4 = load i16, ptr %3, align 2, !dbg !771
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !771
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !771
  ret { i16, i16 } %6, !dbg !771
}

; wasi::lib_generated::fd_allocate
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11fd_allocate17hf95871a71c82ef1eE(i32 %fd, i64 %offset, i64 %len) unnamed_addr #2 !dbg !772 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %len.dbg.spill = alloca [8 x i8], align 8
  %offset.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !776, !DIExpression(), !781)
  store i64 %offset, ptr %offset.dbg.spill, align 8
    #dbg_declare(ptr %offset.dbg.spill, !777, !DIExpression(), !782)
  store i64 %len, ptr %len.dbg.spill, align 8
    #dbg_declare(ptr %len.dbg.spill, !778, !DIExpression(), !783)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_allocate
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_allocate17h306cc9eead141b87E(i32 %fd, i64 %offset, i64 %len) #52, !dbg !784
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !784
    #dbg_declare(ptr %ret.dbg.spill, !779, !DIExpression(), !785)
  %0 = icmp eq i32 %ret, 0, !dbg !786
  br i1 %0, label %bb3, label %bb2, !dbg !786

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !787
  br label %bb4, !dbg !788

bb2:                                              ; preds = %start
  %_9 = trunc i32 %ret to i16, !dbg !789
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !790
  store i16 %_9, ptr %1, align 2, !dbg !790
  store i16 1, ptr %_0, align 2, !dbg !790
  br label %bb4, !dbg !791

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !792
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !792
  %4 = load i16, ptr %3, align 2, !dbg !792
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !792
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !792
  ret { i16, i16 } %6, !dbg !792
}

; wasi::lib_generated::fd_datasync
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11fd_datasync17h896903f285885e8dE(i32 %fd) unnamed_addr #2 !dbg !793 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !797, !DIExpression(), !800)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_datasync
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_datasync17h688f1e6b68c8337dE(i32 %fd) #52, !dbg !801
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !801
    #dbg_declare(ptr %ret.dbg.spill, !798, !DIExpression(), !802)
  %0 = icmp eq i32 %ret, 0, !dbg !803
  br i1 %0, label %bb3, label %bb2, !dbg !803

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !804
  br label %bb4, !dbg !805

bb2:                                              ; preds = %start
  %_5 = trunc i32 %ret to i16, !dbg !806
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !807
  store i16 %_5, ptr %1, align 2, !dbg !807
  store i16 1, ptr %_0, align 2, !dbg !807
  br label %bb4, !dbg !808

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !809
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !809
  %4 = load i16, ptr %3, align 2, !dbg !809
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !809
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !809
  ret { i16, i16 } %6, !dbg !809
}

; wasi::lib_generated::fd_renumber
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11fd_renumber17ha35f9110cd723dc5E(i32 %fd, i32 %to) unnamed_addr #2 !dbg !810 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %to.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !814, !DIExpression(), !818)
  store i32 %to, ptr %to.dbg.spill, align 4
    #dbg_declare(ptr %to.dbg.spill, !815, !DIExpression(), !819)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_renumber
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_renumber17h347a31e5b77196e1E(i32 %fd, i32 %to) #52, !dbg !820
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !820
    #dbg_declare(ptr %ret.dbg.spill, !816, !DIExpression(), !821)
  %0 = icmp eq i32 %ret, 0, !dbg !822
  br i1 %0, label %bb3, label %bb2, !dbg !822

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !823
  br label %bb4, !dbg !824

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !825
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !826
  store i16 %_7, ptr %1, align 2, !dbg !826
  store i16 1, ptr %_0, align 2, !dbg !826
  br label %bb4, !dbg !827

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !828
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !828
  %4 = load i16, ptr %3, align 2, !dbg !828
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !828
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !828
  ret { i16, i16 } %6, !dbg !828
}

; wasi::lib_generated::path_rename
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11path_rename17hecc7b81cf9ee57abE(i32 %fd, ptr align 1 %old_path.0, i32 %old_path.1, i32 %new_fd, ptr align 1 %new_path.0, i32 %new_path.1) unnamed_addr #2 !dbg !829 {
start:
  %self.dbg.spill.i1 = alloca [8 x i8], align 4
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %new_path.dbg.spill = alloca [8 x i8], align 4
  %new_fd.dbg.spill = alloca [4 x i8], align 4
  %old_path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !833, !DIExpression(), !839)
  store ptr %old_path.0, ptr %old_path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %old_path.dbg.spill, i32 4
  store i32 %old_path.1, ptr %0, align 4
    #dbg_declare(ptr %old_path.dbg.spill, !834, !DIExpression(), !840)
  store i32 %new_fd, ptr %new_fd.dbg.spill, align 4
    #dbg_declare(ptr %new_fd.dbg.spill, !835, !DIExpression(), !841)
  store ptr %new_path.0, ptr %new_path.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %new_path.dbg.spill, i32 4
  store i32 %new_path.1, ptr %1, align 4
    #dbg_declare(ptr %new_path.dbg.spill, !836, !DIExpression(), !842)
  store ptr %old_path.0, ptr %self.dbg.spill.i1, align 4
  %2 = getelementptr inbounds i8, ptr %self.dbg.spill.i1, i32 4
  store i32 %old_path.1, ptr %2, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !843, !DIExpression(), !849)
  %_7 = ptrtoint ptr %old_path.0 to i32, !dbg !851
; call core::str::<impl str>::len
  %_10 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %old_path.0, i32 %old_path.1) #52, !dbg !852
  store ptr %new_path.0, ptr %self.dbg.spill.i, align 4
  %3 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %new_path.1, ptr %3, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !853)
  %_12 = ptrtoint ptr %new_path.0 to i32, !dbg !855
; call core::str::<impl str>::len
  %_15 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %new_path.0, i32 %new_path.1) #52, !dbg !856
; call wasi::lib_generated::wasi_snapshot_preview1::path_rename
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111path_rename17h6a4b38a346375893E(i32 %fd, i32 %_7, i32 %_10, i32 %new_fd, i32 %_12, i32 %_15) #52, !dbg !857
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !857
    #dbg_declare(ptr %ret.dbg.spill, !837, !DIExpression(), !858)
  %4 = icmp eq i32 %ret, 0, !dbg !859
  br i1 %4, label %bb7, label %bb6, !dbg !859

bb7:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !860
  br label %bb8, !dbg !861

bb6:                                              ; preds = %start
  %_17 = trunc i32 %ret to i16, !dbg !862
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !863
  store i16 %_17, ptr %5, align 2, !dbg !863
  store i16 1, ptr %_0, align 2, !dbg !863
  br label %bb8, !dbg !864

bb8:                                              ; preds = %bb6, %bb7
  %6 = load i16, ptr %_0, align 2, !dbg !865
  %7 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !865
  %8 = load i16, ptr %7, align 2, !dbg !865
  %9 = insertvalue { i16, i16 } poison, i16 %6, 0, !dbg !865
  %10 = insertvalue { i16, i16 } %9, i16 %8, 1, !dbg !865
  ret { i16, i16 } %10, !dbg !865
}

; wasi::lib_generated::poll_oneoff
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated11poll_oneoff17hb1a91275e741c2fdE(ptr sret([8 x i8]) align 4 %_0, ptr %in_, ptr %out, i32 %nsubscriptions) unnamed_addr #2 !dbg !866 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %nsubscriptions.dbg.spill = alloca [4 x i8], align 4
  %out.dbg.spill = alloca [4 x i8], align 4
  %in_.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store ptr %in_, ptr %in_.dbg.spill, align 4
    #dbg_declare(ptr %in_.dbg.spill, !910, !DIExpression(), !917)
  store ptr %out, ptr %out.dbg.spill, align 4
    #dbg_declare(ptr %out.dbg.spill, !911, !DIExpression(), !918)
  store i32 %nsubscriptions, ptr %nsubscriptions.dbg.spill, align 4
    #dbg_declare(ptr %nsubscriptions.dbg.spill, !912, !DIExpression(), !919)
    #dbg_declare(ptr %rp0, !913, !DIExpression(), !920)
  store i32 undef, ptr %rp0, align 4, !dbg !921
  %_6 = ptrtoint ptr %in_ to i32, !dbg !922
  %_7 = ptrtoint ptr %out to i32, !dbg !923
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !924)
  %_9 = ptrtoint ptr %rp0 to i32, !dbg !926
; call wasi::lib_generated::wasi_snapshot_preview1::poll_oneoff
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111poll_oneoff17h7ab1714565cdb552E(i32 %_6, i32 %_7, i32 %nsubscriptions, i32 %_9) #52, !dbg !927
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !927
    #dbg_declare(ptr %ret.dbg.spill, !915, !DIExpression(), !928)
  %0 = icmp eq i32 %ret, 0, !dbg !929
  br i1 %0, label %bb5, label %bb4, !dbg !929

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !930)
  %_14 = ptrtoint ptr %rp0 to i32, !dbg !932
  %_13 = inttoptr i32 %_14 to ptr, !dbg !932
; call core::ptr::read
  %_12 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_13, ptr align 4 @alloc_d1f5559e557746700ee88ca153e6cc95) #52, !dbg !933
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !934
  store i32 %_12, ptr %1, align 4, !dbg !934
  store i16 0, ptr %_0, align 4, !dbg !934
  br label %bb8, !dbg !935

bb4:                                              ; preds = %start
  %_18 = trunc i32 %ret to i16, !dbg !936
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !937
  store i16 %_18, ptr %2, align 2, !dbg !937
  store i16 1, ptr %_0, align 4, !dbg !937
  br label %bb8, !dbg !938

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !939
}

; wasi::lib_generated::sched_yield
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated11sched_yield17h754331c024719597E() unnamed_addr #2 !dbg !940 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
; call wasi::lib_generated::wasi_snapshot_preview1::sched_yield
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111sched_yield17h2823555b32388fa4E() #52, !dbg !946
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !946
    #dbg_declare(ptr %ret.dbg.spill, !944, !DIExpression(), !947)
  %0 = icmp eq i32 %ret, 0, !dbg !948
  br i1 %0, label %bb3, label %bb2, !dbg !948

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !949
  br label %bb4, !dbg !950

bb2:                                              ; preds = %start
  %_3 = trunc i32 %ret to i16, !dbg !951
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !952
  store i16 %_3, ptr %1, align 2, !dbg !952
  store i16 1, ptr %_0, align 2, !dbg !952
  br label %bb4, !dbg !953

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !954
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !954
  %4 = load i16, ptr %3, align 2, !dbg !954
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !954
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !954
  ret { i16, i16 } %6, !dbg !954
}

; wasi::lib_generated::sock_accept
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated11sock_accept17h300d2bf8b4b573d3E(ptr sret([8 x i8]) align 4 %_0, i32 %fd, i16 %flags) unnamed_addr #2 !dbg !955 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %flags.dbg.spill = alloca [2 x i8], align 2
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !973, !DIExpression(), !986)
  store i16 %flags, ptr %flags.dbg.spill, align 2
    #dbg_declare(ptr %flags.dbg.spill, !974, !DIExpression(), !987)
    #dbg_declare(ptr %rp0, !975, !DIExpression(), !988)
  store i32 undef, ptr %rp0, align 4, !dbg !989
  %_6 = zext i16 %flags to i32, !dbg !990
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !991, !DIExpression(), !999)
  %_7 = ptrtoint ptr %rp0 to i32, !dbg !1001
; call wasi::lib_generated::wasi_snapshot_preview1::sock_accept
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111sock_accept17h1fd793f808c8f883E(i32 %fd, i32 %_6, i32 %_7) #52, !dbg !1002
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1002
    #dbg_declare(ptr %ret.dbg.spill, !984, !DIExpression(), !1003)
  %0 = icmp eq i32 %ret, 0, !dbg !1004
  br i1 %0, label %bb5, label %bb4, !dbg !1004

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !991, !DIExpression(), !1005)
  %_12 = ptrtoint ptr %rp0 to i32, !dbg !1007
  %_11 = inttoptr i32 %_12 to ptr, !dbg !1007
; call core::ptr::read
  %_10 = call i32 @_ZN4core3ptr4read17he4d71e30ba8af448E(ptr %_11, ptr align 4 @alloc_d1bb501f48ae65fbc75ae2ac47791aa5) #52, !dbg !1008
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1009
  store i32 %_10, ptr %1, align 4, !dbg !1009
  store i16 0, ptr %_0, align 4, !dbg !1009
  br label %bb8, !dbg !1010

bb4:                                              ; preds = %start
  %_16 = trunc i32 %ret to i16, !dbg !1011
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1012
  store i16 %_16, ptr %2, align 2, !dbg !1012
  store i16 1, ptr %_0, align 4, !dbg !1012
  br label %bb8, !dbg !1013

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1014
}

; wasi::lib_generated::path_symlink
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated12path_symlink17h382523bc6931237dE(ptr align 1 %old_path.0, i32 %old_path.1, i32 %fd, ptr align 1 %new_path.0, i32 %new_path.1) unnamed_addr #2 !dbg !1015 {
start:
  %self.dbg.spill.i1 = alloca [8 x i8], align 4
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %new_path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %old_path.dbg.spill = alloca [8 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store ptr %old_path.0, ptr %old_path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %old_path.dbg.spill, i32 4
  store i32 %old_path.1, ptr %0, align 4
    #dbg_declare(ptr %old_path.dbg.spill, !1019, !DIExpression(), !1024)
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1020, !DIExpression(), !1025)
  store ptr %new_path.0, ptr %new_path.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %new_path.dbg.spill, i32 4
  store i32 %new_path.1, ptr %1, align 4
    #dbg_declare(ptr %new_path.dbg.spill, !1021, !DIExpression(), !1026)
  store ptr %old_path.0, ptr %self.dbg.spill.i1, align 4
  %2 = getelementptr inbounds i8, ptr %self.dbg.spill.i1, i32 4
  store i32 %old_path.1, ptr %2, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !843, !DIExpression(), !1027)
  %_5 = ptrtoint ptr %old_path.0 to i32, !dbg !1029
; call core::str::<impl str>::len
  %_8 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %old_path.0, i32 %old_path.1) #52, !dbg !1030
  store ptr %new_path.0, ptr %self.dbg.spill.i, align 4
  %3 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %new_path.1, ptr %3, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !1031)
  %_10 = ptrtoint ptr %new_path.0 to i32, !dbg !1033
; call core::str::<impl str>::len
  %_13 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %new_path.0, i32 %new_path.1) #52, !dbg !1034
; call wasi::lib_generated::wasi_snapshot_preview1::path_symlink
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview112path_symlink17h19399a3353cf728fE(i32 %_5, i32 %_8, i32 %fd, i32 %_10, i32 %_13) #52, !dbg !1035
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1035
    #dbg_declare(ptr %ret.dbg.spill, !1022, !DIExpression(), !1036)
  %4 = icmp eq i32 %ret, 0, !dbg !1037
  br i1 %4, label %bb7, label %bb6, !dbg !1037

bb7:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1038
  br label %bb8, !dbg !1039

bb6:                                              ; preds = %start
  %_15 = trunc i32 %ret to i16, !dbg !1040
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1041
  store i16 %_15, ptr %5, align 2, !dbg !1041
  store i16 1, ptr %_0, align 2, !dbg !1041
  br label %bb8, !dbg !1042

bb8:                                              ; preds = %bb6, %bb7
  %6 = load i16, ptr %_0, align 2, !dbg !1043
  %7 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1043
  %8 = load i16, ptr %7, align 2, !dbg !1043
  %9 = insertvalue { i16, i16 } poison, i16 %6, 0, !dbg !1043
  %10 = insertvalue { i16, i16 } %9, i16 %8, 1, !dbg !1043
  ret { i16, i16 } %10, !dbg !1043
}

; wasi::lib_generated::clock_res_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated13clock_res_get17h963bdaa4c47e82c7E(ptr sret([16 x i8]) align 8 %_0, i32 %id) unnamed_addr #2 !dbg !1044 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %id.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [8 x i8], align 8
  store i32 %id, ptr %id.dbg.spill, align 4
    #dbg_declare(ptr %id.dbg.spill, !1062, !DIExpression(), !1074)
    #dbg_declare(ptr %rp0, !1063, !DIExpression(), !1075)
  store i64 undef, ptr %rp0, align 8, !dbg !1076
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1077, !DIExpression(), !1085)
  %_6 = ptrtoint ptr %rp0 to i32, !dbg !1087
; call wasi::lib_generated::wasi_snapshot_preview1::clock_res_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113clock_res_get17h086a49b7a4f55e44E(i32 %id, i32 %_6) #52, !dbg !1088
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1088
    #dbg_declare(ptr %ret.dbg.spill, !1072, !DIExpression(), !1089)
  %0 = icmp eq i32 %ret, 0, !dbg !1090
  br i1 %0, label %bb5, label %bb4, !dbg !1090

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1077, !DIExpression(), !1091)
  %_11 = ptrtoint ptr %rp0 to i32, !dbg !1093
  %_10 = inttoptr i32 %_11 to ptr, !dbg !1093
; call core::ptr::read
  %_9 = call i64 @_ZN4core3ptr4read17h487dc6145fad69b1E(ptr %_10, ptr align 4 @alloc_ee9b84ec0de99e0eb905f593bf15a9be) #52, !dbg !1094
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1095
  store i64 %_9, ptr %1, align 8, !dbg !1095
  store i16 0, ptr %_0, align 8, !dbg !1095
  br label %bb8, !dbg !1096

bb4:                                              ; preds = %start
  %_15 = trunc i32 %ret to i16, !dbg !1097
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1098
  store i16 %_15, ptr %2, align 2, !dbg !1098
  store i16 1, ptr %_0, align 8, !dbg !1098
  br label %bb8, !dbg !1099

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1100
}

; wasi::lib_generated::fd_fdstat_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated13fd_fdstat_get17h5918041547ca7d50E(ptr sret([32 x i8]) align 8 %_0, i32 %fd) unnamed_addr #2 !dbg !1101 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [24 x i8], align 8
  %rp0 = alloca [24 x i8], align 8
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1119, !DIExpression(), !1131)
    #dbg_declare(ptr %rp0, !1120, !DIExpression(), !1132)
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1133, !DIExpression(), !1141)
  %_5 = ptrtoint ptr %rp0 to i32, !dbg !1143
; call wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113fd_fdstat_get17h580d0524d5381556E(i32 %fd, i32 %_5) #52, !dbg !1144
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1144
    #dbg_declare(ptr %ret.dbg.spill, !1129, !DIExpression(), !1145)
  %0 = icmp eq i32 %ret, 0, !dbg !1146
  br i1 %0, label %bb5, label %bb4, !dbg !1146

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1133, !DIExpression(), !1147)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !1149
  %_9 = inttoptr i32 %_10 to ptr, !dbg !1149
; call core::ptr::read
  call void @_ZN4core3ptr4read17h28b571f9788304ccE(ptr sret([24 x i8]) align 8 %_8, ptr %_9, ptr align 4 @alloc_f4d83df641848652ca5a0ee68d0f1a46) #52, !dbg !1150
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1151
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %1, ptr align 8 %_8, i32 24, i1 false), !dbg !1151
  store i16 0, ptr %_0, align 8, !dbg !1151
  br label %bb8, !dbg !1152

bb4:                                              ; preds = %start
  %_14 = trunc i32 %ret to i16, !dbg !1153
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1154
  store i16 %_14, ptr %2, align 2, !dbg !1154
  store i16 1, ptr %_0, align 8, !dbg !1154
  br label %bb8, !dbg !1155

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1156
}

; wasi::lib_generated::path_readlink
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated13path_readlink17hec5e259a0bd2226dE(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 1 %path.0, i32 %path.1, ptr %buf, i32 %buf_len) unnamed_addr #2 !dbg !1157 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %buf_len.dbg.spill = alloca [4 x i8], align 4
  %buf.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1161, !DIExpression(), !1169)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1162, !DIExpression(), !1170)
  store ptr %buf, ptr %buf.dbg.spill, align 4
    #dbg_declare(ptr %buf.dbg.spill, !1163, !DIExpression(), !1171)
  store i32 %buf_len, ptr %buf_len.dbg.spill, align 4
    #dbg_declare(ptr %buf_len.dbg.spill, !1164, !DIExpression(), !1172)
    #dbg_declare(ptr %rp0, !1165, !DIExpression(), !1173)
  store i32 undef, ptr %rp0, align 4, !dbg !1174
  store ptr %path.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !843, !DIExpression(), !1175)
  %_8 = ptrtoint ptr %path.0 to i32, !dbg !1177
; call core::str::<impl str>::len
  %_11 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1178
  %_12 = ptrtoint ptr %buf to i32, !dbg !1179
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !1180)
  %_14 = ptrtoint ptr %rp0 to i32, !dbg !1182
; call wasi::lib_generated::wasi_snapshot_preview1::path_readlink
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113path_readlink17hc076c57d812bf0e0E(i32 %fd, i32 %_8, i32 %_11, i32 %_12, i32 %buf_len, i32 %_14) #52, !dbg !1183
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1183
    #dbg_declare(ptr %ret.dbg.spill, !1167, !DIExpression(), !1184)
  %2 = icmp eq i32 %ret, 0, !dbg !1185
  br i1 %2, label %bb7, label %bb6, !dbg !1185

bb7:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !1186)
  %_19 = ptrtoint ptr %rp0 to i32, !dbg !1188
  %_18 = inttoptr i32 %_19 to ptr, !dbg !1188
; call core::ptr::read
  %_17 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_18, ptr align 4 @alloc_2710d116b7562e6e88e5f569ce3b3314) #52, !dbg !1189
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1190
  store i32 %_17, ptr %3, align 4, !dbg !1190
  store i16 0, ptr %_0, align 4, !dbg !1190
  br label %bb10, !dbg !1191

bb6:                                              ; preds = %start
  %_23 = trunc i32 %ret to i16, !dbg !1192
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1193
  store i16 %_23, ptr %4, align 2, !dbg !1193
  store i16 1, ptr %_0, align 4, !dbg !1193
  br label %bb10, !dbg !1194

bb10:                                             ; preds = %bb6, %bb7
  ret void, !dbg !1195
}

; wasi::lib_generated::sock_shutdown
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated13sock_shutdown17h725fe06176d0b15dE(i32 %fd, i8 %how) unnamed_addr #2 !dbg !1196 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %how.dbg.spill = alloca [1 x i8], align 1
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1200, !DIExpression(), !1204)
  store i8 %how, ptr %how.dbg.spill, align 1
    #dbg_declare(ptr %how.dbg.spill, !1201, !DIExpression(), !1205)
  %_5 = zext i8 %how to i32, !dbg !1206
; call wasi::lib_generated::wasi_snapshot_preview1::sock_shutdown
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113sock_shutdown17h7589812e8ce2a8d2E(i32 %fd, i32 %_5) #52, !dbg !1207
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1207
    #dbg_declare(ptr %ret.dbg.spill, !1202, !DIExpression(), !1208)
  %0 = icmp eq i32 %ret, 0, !dbg !1209
  br i1 %0, label %bb3, label %bb2, !dbg !1209

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1210
  br label %bb4, !dbg !1211

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !1212
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1213
  store i16 %_7, ptr %1, align 2, !dbg !1213
  store i16 1, ptr %_0, align 2, !dbg !1213
  br label %bb4, !dbg !1214

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1215
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1215
  %4 = load i16, ptr %3, align 2, !dbg !1215
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1215
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1215
  ret { i16, i16 } %6, !dbg !1215
}

; wasi::lib_generated::args_sizes_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated14args_sizes_get17h3d2ba99f1b1f0261E(ptr sret([12 x i8]) align 4 %_0) unnamed_addr #2 !dbg !1216 {
start:
  %self.dbg.spill.i3 = alloca [4 x i8], align 4
  %self.dbg.spill.i2 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %rp1 = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
    #dbg_declare(ptr %rp0, !1239, !DIExpression(), !1245)
    #dbg_declare(ptr %rp1, !1241, !DIExpression(), !1246)
  store i32 undef, ptr %rp0, align 4, !dbg !1247
  store i32 undef, ptr %rp1, align 4, !dbg !1248
  store ptr %rp0, ptr %self.dbg.spill.i3, align 4
    #dbg_declare(ptr %self.dbg.spill.i3, !639, !DIExpression(), !1249)
  %_4 = ptrtoint ptr %rp0 to i32, !dbg !1251
  store ptr %rp1, ptr %self.dbg.spill.i2, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !639, !DIExpression(), !1252)
  %_7 = ptrtoint ptr %rp1 to i32, !dbg !1254
; call wasi::lib_generated::wasi_snapshot_preview1::args_sizes_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114args_sizes_get17hf2eb9ec185cc8efeE(i32 %_4, i32 %_7) #52, !dbg !1255
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1255
    #dbg_declare(ptr %ret.dbg.spill, !1243, !DIExpression(), !1256)
  %0 = icmp eq i32 %ret, 0, !dbg !1257
  br i1 %0, label %bb7, label %bb6, !dbg !1257

bb7:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !1258)
  %_13 = ptrtoint ptr %rp0 to i32, !dbg !1260
  %_12 = inttoptr i32 %_13 to ptr, !dbg !1260
; call core::ptr::read
  %_11 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_12, ptr align 4 @alloc_519248ce08ded2655ada4603f97e07fa) #52, !dbg !1261
  store ptr %rp1, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !1262)
  %_18 = ptrtoint ptr %rp1 to i32, !dbg !1264
  %_17 = inttoptr i32 %_18 to ptr, !dbg !1264
; call core::ptr::read
  %_16 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_17, ptr align 4 @alloc_b1a543cfa155ac4cf241ff157a991f7d) #52, !dbg !1265
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1266
  store i32 %_11, ptr %1, align 4, !dbg !1266
  %2 = getelementptr inbounds i8, ptr %1, i32 4, !dbg !1266
  store i32 %_16, ptr %2, align 4, !dbg !1266
  store i16 0, ptr %_0, align 4, !dbg !1266
  br label %bb12, !dbg !1267

bb6:                                              ; preds = %start
  %_22 = trunc i32 %ret to i16, !dbg !1268
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1269
  store i16 %_22, ptr %3, align 2, !dbg !1269
  store i16 1, ptr %_0, align 4, !dbg !1269
  br label %bb12, !dbg !1270

bb12:                                             ; preds = %bb6, %bb7
  ret void, !dbg !1271
}

; wasi::lib_generated::clock_time_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated14clock_time_get17hf3af7872ec7af954E(ptr sret([16 x i8]) align 8 %_0, i32 %id, i64 %precision) unnamed_addr #2 !dbg !1272 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %precision.dbg.spill = alloca [8 x i8], align 8
  %id.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [8 x i8], align 8
  store i32 %id, ptr %id.dbg.spill, align 4
    #dbg_declare(ptr %id.dbg.spill, !1276, !DIExpression(), !1282)
  store i64 %precision, ptr %precision.dbg.spill, align 8
    #dbg_declare(ptr %precision.dbg.spill, !1277, !DIExpression(), !1283)
    #dbg_declare(ptr %rp0, !1278, !DIExpression(), !1284)
  store i64 undef, ptr %rp0, align 8, !dbg !1285
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1077, !DIExpression(), !1286)
  %_8 = ptrtoint ptr %rp0 to i32, !dbg !1288
; call wasi::lib_generated::wasi_snapshot_preview1::clock_time_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114clock_time_get17h4dc8e8bee1835d0cE(i32 %id, i64 %precision, i32 %_8) #52, !dbg !1289
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1289
    #dbg_declare(ptr %ret.dbg.spill, !1280, !DIExpression(), !1290)
  %0 = icmp eq i32 %ret, 0, !dbg !1291
  br i1 %0, label %bb5, label %bb4, !dbg !1291

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1077, !DIExpression(), !1292)
  %_13 = ptrtoint ptr %rp0 to i32, !dbg !1294
  %_12 = inttoptr i32 %_13 to ptr, !dbg !1294
; call core::ptr::read
  %_11 = call i64 @_ZN4core3ptr4read17h487dc6145fad69b1E(ptr %_12, ptr align 4 @alloc_123b1cf35884e2572af9f60d83db2bd9) #52, !dbg !1295
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1296
  store i64 %_11, ptr %1, align 8, !dbg !1296
  store i16 0, ptr %_0, align 8, !dbg !1296
  br label %bb8, !dbg !1297

bb4:                                              ; preds = %start
  %_17 = trunc i32 %ret to i16, !dbg !1298
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1299
  store i16 %_17, ptr %2, align 2, !dbg !1299
  store i16 1, ptr %_0, align 8, !dbg !1299
  br label %bb8, !dbg !1300

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1301
}

; wasi::lib_generated::fd_prestat_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated14fd_prestat_get17hcb4b9e9d217424b3E(ptr sret([12 x i8]) align 4 %_0, i32 %fd) unnamed_addr #2 !dbg !1302 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [8 x i8], align 4
  %rp0 = alloca [8 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1320, !DIExpression(), !1332)
    #dbg_declare(ptr %rp0, !1321, !DIExpression(), !1333)
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1334, !DIExpression(), !1342)
  %_5 = ptrtoint ptr %rp0 to i32, !dbg !1344
; call wasi::lib_generated::wasi_snapshot_preview1::fd_prestat_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114fd_prestat_get17hc6fcf80e1c03dd6bE(i32 %fd, i32 %_5) #52, !dbg !1345
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1345
    #dbg_declare(ptr %ret.dbg.spill, !1330, !DIExpression(), !1346)
  %0 = icmp eq i32 %ret, 0, !dbg !1347
  br i1 %0, label %bb5, label %bb4, !dbg !1347

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1334, !DIExpression(), !1348)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !1350
  %_9 = inttoptr i32 %_10 to ptr, !dbg !1350
; call core::ptr::read
  call void @_ZN4core3ptr4read17h58671998979cdf8bE(ptr sret([8 x i8]) align 4 %_8, ptr %_9, ptr align 4 @alloc_7d502150ec9455fc6ba4e8bf98dc17d0) #52, !dbg !1351
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1352
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %1, ptr align 4 %_8, i32 8, i1 false), !dbg !1352
  store i16 0, ptr %_0, align 4, !dbg !1352
  br label %bb8, !dbg !1353

bb4:                                              ; preds = %start
  %_14 = trunc i32 %ret to i16, !dbg !1354
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1355
  store i16 %_14, ptr %2, align 2, !dbg !1355
  store i16 1, ptr %_0, align 4, !dbg !1355
  br label %bb8, !dbg !1356

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1357
}

; wasi::lib_generated::fd_filestat_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated15fd_filestat_get17h0bad099bde016b06E(ptr sret([72 x i8]) align 8 %_0, i32 %fd) unnamed_addr #2 !dbg !1358 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_8 = alloca [64 x i8], align 8
  %rp0 = alloca [64 x i8], align 8
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1376, !DIExpression(), !1388)
    #dbg_declare(ptr %rp0, !1377, !DIExpression(), !1389)
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1390, !DIExpression(), !1398)
  %_5 = ptrtoint ptr %rp0 to i32, !dbg !1400
; call wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview115fd_filestat_get17h501df1c79e23b9ffE(i32 %fd, i32 %_5) #52, !dbg !1401
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1401
    #dbg_declare(ptr %ret.dbg.spill, !1386, !DIExpression(), !1402)
  %0 = icmp eq i32 %ret, 0, !dbg !1403
  br i1 %0, label %bb5, label %bb4, !dbg !1403

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1390, !DIExpression(), !1404)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !1406
  %_9 = inttoptr i32 %_10 to ptr, !dbg !1406
; call core::ptr::read
  call void @_ZN4core3ptr4read17hc08313af9d479144E(ptr sret([64 x i8]) align 8 %_8, ptr %_9, ptr align 4 @alloc_11b7520103d053307d60012511f4ff54) #52, !dbg !1407
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1408
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %1, ptr align 8 %_8, i32 64, i1 false), !dbg !1408
  store i16 0, ptr %_0, align 8, !dbg !1408
  br label %bb8, !dbg !1409

bb4:                                              ; preds = %start
  %_14 = trunc i32 %ret to i16, !dbg !1410
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1411
  store i16 %_14, ptr %2, align 2, !dbg !1411
  store i16 1, ptr %_0, align 8, !dbg !1411
  br label %bb8, !dbg !1412

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !1413
}

; wasi::lib_generated::path_unlink_file
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated16path_unlink_file17hfd3c14cbf52140efE(i32 %fd, ptr align 1 %path.0, i32 %path.1) unnamed_addr #2 !dbg !1414 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1418, !DIExpression(), !1422)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1419, !DIExpression(), !1423)
  store ptr %path.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !1424)
  %_5 = ptrtoint ptr %path.0 to i32, !dbg !1426
; call core::str::<impl str>::len
  %_8 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1427
; call wasi::lib_generated::wasi_snapshot_preview1::path_unlink_file
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview116path_unlink_file17hacac4e766be441c4E(i32 %fd, i32 %_5, i32 %_8) #52, !dbg !1428
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1428
    #dbg_declare(ptr %ret.dbg.spill, !1420, !DIExpression(), !1429)
  %2 = icmp eq i32 %ret, 0, !dbg !1430
  br i1 %2, label %bb5, label %bb4, !dbg !1430

bb5:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1431
  br label %bb6, !dbg !1432

bb4:                                              ; preds = %start
  %_10 = trunc i32 %ret to i16, !dbg !1433
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1434
  store i16 %_10, ptr %3, align 2, !dbg !1434
  store i16 1, ptr %_0, align 2, !dbg !1434
  br label %bb6, !dbg !1435

bb6:                                              ; preds = %bb4, %bb5
  %4 = load i16, ptr %_0, align 2, !dbg !1436
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1436
  %6 = load i16, ptr %5, align 2, !dbg !1436
  %7 = insertvalue { i16, i16 } poison, i16 %4, 0, !dbg !1436
  %8 = insertvalue { i16, i16 } %7, i16 %6, 1, !dbg !1436
  ret { i16, i16 } %8, !dbg !1436
}

; wasi::lib_generated::environ_sizes_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated17environ_sizes_get17haf8697964351a6a8E(ptr sret([12 x i8]) align 4 %_0) unnamed_addr #2 !dbg !1437 {
start:
  %self.dbg.spill.i3 = alloca [4 x i8], align 4
  %self.dbg.spill.i2 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %rp1 = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
    #dbg_declare(ptr %rp0, !1439, !DIExpression(), !1445)
    #dbg_declare(ptr %rp1, !1441, !DIExpression(), !1446)
  store i32 undef, ptr %rp0, align 4, !dbg !1447
  store i32 undef, ptr %rp1, align 4, !dbg !1448
  store ptr %rp0, ptr %self.dbg.spill.i3, align 4
    #dbg_declare(ptr %self.dbg.spill.i3, !639, !DIExpression(), !1449)
  %_4 = ptrtoint ptr %rp0 to i32, !dbg !1451
  store ptr %rp1, ptr %self.dbg.spill.i2, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !639, !DIExpression(), !1452)
  %_7 = ptrtoint ptr %rp1 to i32, !dbg !1454
; call wasi::lib_generated::wasi_snapshot_preview1::environ_sizes_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview117environ_sizes_get17h3870c602c4670daeE(i32 %_4, i32 %_7) #52, !dbg !1455
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1455
    #dbg_declare(ptr %ret.dbg.spill, !1443, !DIExpression(), !1456)
  %0 = icmp eq i32 %ret, 0, !dbg !1457
  br i1 %0, label %bb7, label %bb6, !dbg !1457

bb7:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !1458)
  %_13 = ptrtoint ptr %rp0 to i32, !dbg !1460
  %_12 = inttoptr i32 %_13 to ptr, !dbg !1460
; call core::ptr::read
  %_11 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_12, ptr align 4 @alloc_e122aba4a1aab03052298bf45e3e24db) #52, !dbg !1461
  store ptr %rp1, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !1462)
  %_18 = ptrtoint ptr %rp1 to i32, !dbg !1464
  %_17 = inttoptr i32 %_18 to ptr, !dbg !1464
; call core::ptr::read
  %_16 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_17, ptr align 4 @alloc_7c8f1f6de8ed387b2a927003a7b97c49) #52, !dbg !1465
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1466
  store i32 %_11, ptr %1, align 4, !dbg !1466
  %2 = getelementptr inbounds i8, ptr %1, i32 4, !dbg !1466
  store i32 %_16, ptr %2, align 4, !dbg !1466
  store i16 0, ptr %_0, align 4, !dbg !1466
  br label %bb12, !dbg !1467

bb6:                                              ; preds = %start
  %_22 = trunc i32 %ret to i16, !dbg !1468
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1469
  store i16 %_22, ptr %3, align 2, !dbg !1469
  store i16 1, ptr %_0, align 4, !dbg !1469
  br label %bb12, !dbg !1470

bb12:                                             ; preds = %bb6, %bb7
  ret void, !dbg !1471
}

; wasi::lib_generated::path_filestat_get
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated17path_filestat_get17h36e722cead09cc29E(ptr sret([72 x i8]) align 8 %_0, i32 %fd, i32 %flags, ptr align 1 %path.0, i32 %path.1) unnamed_addr #2 !dbg !1472 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [8 x i8], align 4
  %flags.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_15 = alloca [64 x i8], align 8
  %rp0 = alloca [64 x i8], align 8
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1476, !DIExpression(), !1483)
  store i32 %flags, ptr %flags.dbg.spill, align 4
    #dbg_declare(ptr %flags.dbg.spill, !1477, !DIExpression(), !1484)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1478, !DIExpression(), !1485)
    #dbg_declare(ptr %rp0, !1479, !DIExpression(), !1486)
  store ptr %path.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !843, !DIExpression(), !1487)
  %_8 = ptrtoint ptr %path.0 to i32, !dbg !1489
; call core::str::<impl str>::len
  %_11 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1490
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1390, !DIExpression(), !1491)
  %_12 = ptrtoint ptr %rp0 to i32, !dbg !1493
; call wasi::lib_generated::wasi_snapshot_preview1::path_filestat_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview117path_filestat_get17h721971769c2cf374E(i32 %fd, i32 %flags, i32 %_8, i32 %_11, i32 %_12) #52, !dbg !1494
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1494
    #dbg_declare(ptr %ret.dbg.spill, !1481, !DIExpression(), !1495)
  %2 = icmp eq i32 %ret, 0, !dbg !1496
  br i1 %2, label %bb7, label %bb6, !dbg !1496

bb7:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1390, !DIExpression(), !1497)
  %_17 = ptrtoint ptr %rp0 to i32, !dbg !1499
  %_16 = inttoptr i32 %_17 to ptr, !dbg !1499
; call core::ptr::read
  call void @_ZN4core3ptr4read17hc08313af9d479144E(ptr sret([64 x i8]) align 8 %_15, ptr %_16, ptr align 4 @alloc_ae2bd5de5116fcab94796dd68c9bb77c) #52, !dbg !1500
  %3 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !1501
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %3, ptr align 8 %_15, i32 64, i1 false), !dbg !1501
  store i16 0, ptr %_0, align 8, !dbg !1501
  br label %bb10, !dbg !1502

bb6:                                              ; preds = %start
  %_21 = trunc i32 %ret to i16, !dbg !1503
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1504
  store i16 %_21, ptr %4, align 2, !dbg !1504
  store i16 1, ptr %_0, align 8, !dbg !1504
  br label %bb10, !dbg !1505

bb10:                                             ; preds = %bb6, %bb7
  ret void, !dbg !1506
}

; wasi::lib_generated::fd_fdstat_set_flags
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated19fd_fdstat_set_flags17h61a62bf5e192b128E(i32 %fd, i16 %flags) unnamed_addr #2 !dbg !1507 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %flags.dbg.spill = alloca [2 x i8], align 2
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1511, !DIExpression(), !1515)
  store i16 %flags, ptr %flags.dbg.spill, align 2
    #dbg_declare(ptr %flags.dbg.spill, !1512, !DIExpression(), !1516)
  %_5 = zext i16 %flags to i32, !dbg !1517
; call wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_set_flags
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview119fd_fdstat_set_flags17h1498b47897c0a35bE(i32 %fd, i32 %_5) #52, !dbg !1518
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1518
    #dbg_declare(ptr %ret.dbg.spill, !1513, !DIExpression(), !1519)
  %0 = icmp eq i32 %ret, 0, !dbg !1520
  br i1 %0, label %bb3, label %bb2, !dbg !1520

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1521
  br label %bb4, !dbg !1522

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !1523
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1524
  store i16 %_7, ptr %1, align 2, !dbg !1524
  store i16 1, ptr %_0, align 2, !dbg !1524
  br label %bb4, !dbg !1525

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1526
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1526
  %4 = load i16, ptr %3, align 2, !dbg !1526
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1526
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1526
  ret { i16, i16 } %6, !dbg !1526
}

; wasi::lib_generated::fd_prestat_dir_name
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated19fd_prestat_dir_name17h86844aaa09e395d0E(i32 %fd, ptr %path, i32 %path_len) unnamed_addr #2 !dbg !1527 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %path_len.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1531, !DIExpression(), !1536)
  store ptr %path, ptr %path.dbg.spill, align 4
    #dbg_declare(ptr %path.dbg.spill, !1532, !DIExpression(), !1537)
  store i32 %path_len, ptr %path_len.dbg.spill, align 4
    #dbg_declare(ptr %path_len.dbg.spill, !1533, !DIExpression(), !1538)
  %_6 = ptrtoint ptr %path to i32, !dbg !1539
; call wasi::lib_generated::wasi_snapshot_preview1::fd_prestat_dir_name
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview119fd_prestat_dir_name17h7b043e4708de23a4E(i32 %fd, i32 %_6, i32 %path_len) #52, !dbg !1540
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1540
    #dbg_declare(ptr %ret.dbg.spill, !1534, !DIExpression(), !1541)
  %0 = icmp eq i32 %ret, 0, !dbg !1542
  br i1 %0, label %bb3, label %bb2, !dbg !1542

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1543
  br label %bb4, !dbg !1544

bb2:                                              ; preds = %start
  %_9 = trunc i32 %ret to i16, !dbg !1545
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1546
  store i16 %_9, ptr %1, align 2, !dbg !1546
  store i16 1, ptr %_0, align 2, !dbg !1546
  br label %bb4, !dbg !1547

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1548
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1548
  %4 = load i16, ptr %3, align 2, !dbg !1548
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1548
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1548
  ret { i16, i16 } %6, !dbg !1548
}

; wasi::lib_generated::fd_fdstat_set_rights
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated20fd_fdstat_set_rights17h3601f3e6d2a09b9bE(i32 %fd, i64 %fs_rights_base, i64 %fs_rights_inheriting) unnamed_addr #2 !dbg !1549 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fs_rights_inheriting.dbg.spill = alloca [8 x i8], align 8
  %fs_rights_base.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1551, !DIExpression(), !1556)
  store i64 %fs_rights_base, ptr %fs_rights_base.dbg.spill, align 8
    #dbg_declare(ptr %fs_rights_base.dbg.spill, !1552, !DIExpression(), !1557)
  store i64 %fs_rights_inheriting, ptr %fs_rights_inheriting.dbg.spill, align 8
    #dbg_declare(ptr %fs_rights_inheriting.dbg.spill, !1553, !DIExpression(), !1558)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_set_rights
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview120fd_fdstat_set_rights17h0c3f8a2925c7d2c0E(i32 %fd, i64 %fs_rights_base, i64 %fs_rights_inheriting) #52, !dbg !1559
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1559
    #dbg_declare(ptr %ret.dbg.spill, !1554, !DIExpression(), !1560)
  %0 = icmp eq i32 %ret, 0, !dbg !1561
  br i1 %0, label %bb3, label %bb2, !dbg !1561

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1562
  br label %bb4, !dbg !1563

bb2:                                              ; preds = %start
  %_9 = trunc i32 %ret to i16, !dbg !1564
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1565
  store i16 %_9, ptr %1, align 2, !dbg !1565
  store i16 1, ptr %_0, align 2, !dbg !1565
  br label %bb4, !dbg !1566

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1567
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1567
  %4 = load i16, ptr %3, align 2, !dbg !1567
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1567
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1567
  ret { i16, i16 } %6, !dbg !1567
}

; wasi::lib_generated::fd_filestat_set_size
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated20fd_filestat_set_size17hbd7e7495799fbe2fE(i32 %fd, i64 %size) unnamed_addr #2 !dbg !1568 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %size.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1572, !DIExpression(), !1576)
  store i64 %size, ptr %size.dbg.spill, align 8
    #dbg_declare(ptr %size.dbg.spill, !1573, !DIExpression(), !1577)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_set_size
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview120fd_filestat_set_size17h7ed0a5f28827e26aE(i32 %fd, i64 %size) #52, !dbg !1578
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1578
    #dbg_declare(ptr %ret.dbg.spill, !1574, !DIExpression(), !1579)
  %0 = icmp eq i32 %ret, 0, !dbg !1580
  br i1 %0, label %bb3, label %bb2, !dbg !1580

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1581
  br label %bb4, !dbg !1582

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !1583
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1584
  store i16 %_7, ptr %1, align 2, !dbg !1584
  store i16 1, ptr %_0, align 2, !dbg !1584
  br label %bb4, !dbg !1585

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1586
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1586
  %4 = load i16, ptr %3, align 2, !dbg !1586
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1586
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1586
  ret { i16, i16 } %6, !dbg !1586
}

; wasi::lib_generated::fd_filestat_set_times
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated21fd_filestat_set_times17hb6ca76dc5c47ded6E(i32 %fd, i64 %atim, i64 %mtim, i16 %fst_flags) unnamed_addr #2 !dbg !1587 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fst_flags.dbg.spill = alloca [2 x i8], align 2
  %mtim.dbg.spill = alloca [8 x i8], align 8
  %atim.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1591, !DIExpression(), !1597)
  store i64 %atim, ptr %atim.dbg.spill, align 8
    #dbg_declare(ptr %atim.dbg.spill, !1592, !DIExpression(), !1598)
  store i64 %mtim, ptr %mtim.dbg.spill, align 8
    #dbg_declare(ptr %mtim.dbg.spill, !1593, !DIExpression(), !1599)
  store i16 %fst_flags, ptr %fst_flags.dbg.spill, align 2
    #dbg_declare(ptr %fst_flags.dbg.spill, !1594, !DIExpression(), !1600)
  %_9 = zext i16 %fst_flags to i32, !dbg !1601
; call wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_set_times
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121fd_filestat_set_times17h6a0126c21b0af73dE(i32 %fd, i64 %atim, i64 %mtim, i32 %_9) #52, !dbg !1602
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1602
    #dbg_declare(ptr %ret.dbg.spill, !1595, !DIExpression(), !1603)
  %0 = icmp eq i32 %ret, 0, !dbg !1604
  br i1 %0, label %bb3, label %bb2, !dbg !1604

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1605
  br label %bb4, !dbg !1606

bb2:                                              ; preds = %start
  %_11 = trunc i32 %ret to i16, !dbg !1607
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1608
  store i16 %_11, ptr %1, align 2, !dbg !1608
  store i16 1, ptr %_0, align 2, !dbg !1608
  br label %bb4, !dbg !1609

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !1610
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1610
  %4 = load i16, ptr %3, align 2, !dbg !1610
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !1610
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !1610
  ret { i16, i16 } %6, !dbg !1610
}

; wasi::lib_generated::path_create_directory
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated21path_create_directory17h51aa6b558786c7beE(i32 %fd, ptr align 1 %path.0, i32 %path.1) unnamed_addr #2 !dbg !1611 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1613, !DIExpression(), !1617)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1614, !DIExpression(), !1618)
  store ptr %path.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !1619)
  %_5 = ptrtoint ptr %path.0 to i32, !dbg !1621
; call core::str::<impl str>::len
  %_8 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1622
; call wasi::lib_generated::wasi_snapshot_preview1::path_create_directory
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121path_create_directory17h043ddd360752c37aE(i32 %fd, i32 %_5, i32 %_8) #52, !dbg !1623
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1623
    #dbg_declare(ptr %ret.dbg.spill, !1615, !DIExpression(), !1624)
  %2 = icmp eq i32 %ret, 0, !dbg !1625
  br i1 %2, label %bb5, label %bb4, !dbg !1625

bb5:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1626
  br label %bb6, !dbg !1627

bb4:                                              ; preds = %start
  %_10 = trunc i32 %ret to i16, !dbg !1628
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1629
  store i16 %_10, ptr %3, align 2, !dbg !1629
  store i16 1, ptr %_0, align 2, !dbg !1629
  br label %bb6, !dbg !1630

bb6:                                              ; preds = %bb4, %bb5
  %4 = load i16, ptr %_0, align 2, !dbg !1631
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1631
  %6 = load i16, ptr %5, align 2, !dbg !1631
  %7 = insertvalue { i16, i16 } poison, i16 %4, 0, !dbg !1631
  %8 = insertvalue { i16, i16 } %7, i16 %6, 1, !dbg !1631
  ret { i16, i16 } %8, !dbg !1631
}

; wasi::lib_generated::path_remove_directory
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated21path_remove_directory17h454076b0e3db5ad7E(i32 %fd, ptr align 1 %path.0, i32 %path.1) unnamed_addr #2 !dbg !1632 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %path.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1634, !DIExpression(), !1638)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1635, !DIExpression(), !1639)
  store ptr %path.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !1640)
  %_5 = ptrtoint ptr %path.0 to i32, !dbg !1642
; call core::str::<impl str>::len
  %_8 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1643
; call wasi::lib_generated::wasi_snapshot_preview1::path_remove_directory
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121path_remove_directory17hdcdfe2f9bcd9fc06E(i32 %fd, i32 %_5, i32 %_8) #52, !dbg !1644
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1644
    #dbg_declare(ptr %ret.dbg.spill, !1636, !DIExpression(), !1645)
  %2 = icmp eq i32 %ret, 0, !dbg !1646
  br i1 %2, label %bb5, label %bb4, !dbg !1646

bb5:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1647
  br label %bb6, !dbg !1648

bb4:                                              ; preds = %start
  %_10 = trunc i32 %ret to i16, !dbg !1649
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1650
  store i16 %_10, ptr %3, align 2, !dbg !1650
  store i16 1, ptr %_0, align 2, !dbg !1650
  br label %bb6, !dbg !1651

bb6:                                              ; preds = %bb4, %bb5
  %4 = load i16, ptr %_0, align 2, !dbg !1652
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1652
  %6 = load i16, ptr %5, align 2, !dbg !1652
  %7 = insertvalue { i16, i16 } poison, i16 %4, 0, !dbg !1652
  %8 = insertvalue { i16, i16 } %7, i16 %6, 1, !dbg !1652
  ret { i16, i16 } %8, !dbg !1652
}

; wasi::lib_generated::path_filestat_set_times
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated23path_filestat_set_times17h14a5913f2496ec36E(i32 %fd, i32 %flags, ptr align 1 %path.0, i32 %path.1, i64 %atim, i64 %mtim, i16 %fst_flags) unnamed_addr #2 !dbg !1653 {
start:
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fst_flags.dbg.spill = alloca [2 x i8], align 2
  %mtim.dbg.spill = alloca [8 x i8], align 8
  %atim.dbg.spill = alloca [8 x i8], align 8
  %path.dbg.spill = alloca [8 x i8], align 4
  %flags.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !1657, !DIExpression(), !1665)
  store i32 %flags, ptr %flags.dbg.spill, align 4
    #dbg_declare(ptr %flags.dbg.spill, !1658, !DIExpression(), !1666)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !1659, !DIExpression(), !1667)
  store i64 %atim, ptr %atim.dbg.spill, align 8
    #dbg_declare(ptr %atim.dbg.spill, !1660, !DIExpression(), !1668)
  store i64 %mtim, ptr %mtim.dbg.spill, align 8
    #dbg_declare(ptr %mtim.dbg.spill, !1661, !DIExpression(), !1669)
  store i16 %fst_flags, ptr %fst_flags.dbg.spill, align 2
    #dbg_declare(ptr %fst_flags.dbg.spill, !1662, !DIExpression(), !1670)
  store ptr %path.0, ptr %self.dbg.spill.i, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !1671)
  %_10 = ptrtoint ptr %path.0 to i32, !dbg !1673
; call core::str::<impl str>::len
  %_13 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !1674
  %_16 = zext i16 %fst_flags to i32, !dbg !1675
; call wasi::lib_generated::wasi_snapshot_preview1::path_filestat_set_times
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview123path_filestat_set_times17h49b1faeed91aeeccE(i32 %fd, i32 %flags, i32 %_10, i32 %_13, i64 %atim, i64 %mtim, i32 %_16) #52, !dbg !1676
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !1676
    #dbg_declare(ptr %ret.dbg.spill, !1663, !DIExpression(), !1677)
  %2 = icmp eq i32 %ret, 0, !dbg !1678
  br i1 %2, label %bb5, label %bb4, !dbg !1678

bb5:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !1679
  br label %bb6, !dbg !1680

bb4:                                              ; preds = %start
  %_18 = trunc i32 %ret to i16, !dbg !1681
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1682
  store i16 %_18, ptr %3, align 2, !dbg !1682
  store i16 1, ptr %_0, align 2, !dbg !1682
  br label %bb6, !dbg !1683

bb6:                                              ; preds = %bb4, %bb5
  %4 = load i16, ptr %_0, align 2, !dbg !1684
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !1684
  %6 = load i16, ptr %5, align 2, !dbg !1684
  %7 = insertvalue { i16, i16 } poison, i16 %4, 0, !dbg !1684
  %8 = insertvalue { i16, i16 } %7, i16 %6, 1, !dbg !1684
  ret { i16, i16 } %8, !dbg !1684
}

; wasi::lib_generated::Errno::raw
; Function Attrs: nounwind
define dso_local i16 @_ZN4wasi13lib_generated5Errno3raw17h718b90db54a58ce5E(ptr align 2 %self) unnamed_addr #2 !dbg !1685 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1691, !DIExpression(), !1692)
  %_0 = load i16, ptr %self, align 2, !dbg !1693
  ret i16 %_0, !dbg !1694
}

; wasi::lib_generated::Errno::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated5Errno4name17h3a1ad9514045c510E(ptr align 2 %self) unnamed_addr #2 !dbg !1695 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1700, !DIExpression(), !1701)
  %0 = load i16, ptr %self, align 2, !dbg !1702
  switch i16 %0, label %bb1 [
    i16 0, label %bb78
    i16 1, label %bb77
    i16 2, label %bb76
    i16 3, label %bb75
    i16 4, label %bb74
    i16 5, label %bb73
    i16 6, label %bb72
    i16 7, label %bb71
    i16 8, label %bb70
    i16 9, label %bb69
    i16 10, label %bb68
    i16 11, label %bb67
    i16 12, label %bb66
    i16 13, label %bb65
    i16 14, label %bb64
    i16 15, label %bb63
    i16 16, label %bb62
    i16 17, label %bb61
    i16 18, label %bb60
    i16 19, label %bb59
    i16 20, label %bb58
    i16 21, label %bb57
    i16 22, label %bb56
    i16 23, label %bb55
    i16 24, label %bb54
    i16 25, label %bb53
    i16 26, label %bb52
    i16 27, label %bb51
    i16 28, label %bb50
    i16 29, label %bb49
    i16 30, label %bb48
    i16 31, label %bb47
    i16 32, label %bb46
    i16 33, label %bb45
    i16 34, label %bb44
    i16 35, label %bb43
    i16 36, label %bb42
    i16 37, label %bb41
    i16 38, label %bb40
    i16 39, label %bb39
    i16 40, label %bb38
    i16 41, label %bb37
    i16 42, label %bb36
    i16 43, label %bb35
    i16 44, label %bb34
    i16 45, label %bb33
    i16 46, label %bb32
    i16 47, label %bb31
    i16 48, label %bb30
    i16 49, label %bb29
    i16 50, label %bb28
    i16 51, label %bb27
    i16 52, label %bb26
    i16 53, label %bb25
    i16 54, label %bb24
    i16 55, label %bb23
    i16 56, label %bb22
    i16 57, label %bb21
    i16 58, label %bb20
    i16 59, label %bb19
    i16 60, label %bb18
    i16 61, label %bb17
    i16 62, label %bb16
    i16 63, label %bb15
    i16 64, label %bb14
    i16 65, label %bb13
    i16 66, label %bb12
    i16 67, label %bb11
    i16 68, label %bb10
    i16 69, label %bb9
    i16 70, label %bb8
    i16 71, label %bb7
    i16 72, label %bb6
    i16 73, label %bb5
    i16 74, label %bb4
    i16 75, label %bb3
    i16 76, label %bb2
  ], !dbg !1702

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_f41208f7f9aabadef03af3b7c3757409) #53, !dbg !1703
  unreachable, !dbg !1703

bb78:                                             ; preds = %start
  store ptr @alloc_1713696202b080e762d75726f96ce4bd, ptr %_0, align 4, !dbg !1704
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1704
  store i32 7, ptr %1, align 4, !dbg !1704
  br label %bb79, !dbg !1704

bb77:                                             ; preds = %start
  store ptr @alloc_71a2b09e7c042f78ec615d7f2ca98c1b, ptr %_0, align 4, !dbg !1705
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1705
  store i32 4, ptr %2, align 4, !dbg !1705
  br label %bb79, !dbg !1705

bb76:                                             ; preds = %start
  store ptr @alloc_4981db4d4af768681fc505432fb27604, ptr %_0, align 4, !dbg !1706
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1706
  store i32 5, ptr %3, align 4, !dbg !1706
  br label %bb79, !dbg !1706

bb75:                                             ; preds = %start
  store ptr @alloc_679cc6e775da6c924f4730aef593815b, ptr %_0, align 4, !dbg !1707
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1707
  store i32 9, ptr %4, align 4, !dbg !1707
  br label %bb79, !dbg !1707

bb74:                                             ; preds = %start
  store ptr @alloc_e46aa89b757d2be1eb1cb75b417933df, ptr %_0, align 4, !dbg !1708
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1708
  store i32 12, ptr %5, align 4, !dbg !1708
  br label %bb79, !dbg !1708

bb73:                                             ; preds = %start
  store ptr @alloc_d9627941932eaa106b478dfe15f39b87, ptr %_0, align 4, !dbg !1709
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1709
  store i32 11, ptr %6, align 4, !dbg !1709
  br label %bb79, !dbg !1709

bb72:                                             ; preds = %start
  store ptr @alloc_c2f6cff0c49c8796b95e84de6cf7193f, ptr %_0, align 4, !dbg !1710
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1710
  store i32 5, ptr %7, align 4, !dbg !1710
  br label %bb79, !dbg !1710

bb71:                                             ; preds = %start
  store ptr @alloc_c74b81658384824d0c279164dac5652c, ptr %_0, align 4, !dbg !1711
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1711
  store i32 7, ptr %8, align 4, !dbg !1711
  br label %bb79, !dbg !1711

bb70:                                             ; preds = %start
  store ptr @alloc_e189066c7ac3daf6ee8ec65942018cc7, ptr %_0, align 4, !dbg !1712
  %9 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1712
  store i32 4, ptr %9, align 4, !dbg !1712
  br label %bb79, !dbg !1712

bb69:                                             ; preds = %start
  store ptr @alloc_ca66d8218ea61ea49e67cd560b7a6765, ptr %_0, align 4, !dbg !1713
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1713
  store i32 6, ptr %10, align 4, !dbg !1713
  br label %bb79, !dbg !1713

bb68:                                             ; preds = %start
  store ptr @alloc_b5396060cb2bc4d6a8892b062bdaa563, ptr %_0, align 4, !dbg !1714
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1714
  store i32 4, ptr %11, align 4, !dbg !1714
  br label %bb79, !dbg !1714

bb67:                                             ; preds = %start
  store ptr @alloc_b5e9df1b543249147682414a6f297f1f, ptr %_0, align 4, !dbg !1715
  %12 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1715
  store i32 8, ptr %12, align 4, !dbg !1715
  br label %bb79, !dbg !1715

bb66:                                             ; preds = %start
  store ptr @alloc_7e757e385d3a69379f474707734cf7ae, ptr %_0, align 4, !dbg !1716
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1716
  store i32 5, ptr %13, align 4, !dbg !1716
  br label %bb79, !dbg !1716

bb65:                                             ; preds = %start
  store ptr @alloc_466b9e6b1a6abc26631d981143acb197, ptr %_0, align 4, !dbg !1717
  %14 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1717
  store i32 11, ptr %14, align 4, !dbg !1717
  br label %bb79, !dbg !1717

bb64:                                             ; preds = %start
  store ptr @alloc_7475bd4428214e92b4dc1203d04ce21a, ptr %_0, align 4, !dbg !1718
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1718
  store i32 11, ptr %15, align 4, !dbg !1718
  br label %bb79, !dbg !1718

bb63:                                             ; preds = %start
  store ptr @alloc_5680db10ffd639594b86a5171e1323e2, ptr %_0, align 4, !dbg !1719
  %16 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1719
  store i32 9, ptr %16, align 4, !dbg !1719
  br label %bb79, !dbg !1719

bb62:                                             ; preds = %start
  store ptr @alloc_9e91c6ecbddbf8ac8b47f0d0cfe27259, ptr %_0, align 4, !dbg !1720
  %17 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1720
  store i32 6, ptr %17, align 4, !dbg !1720
  br label %bb79, !dbg !1720

bb61:                                             ; preds = %start
  store ptr @alloc_e45387ccd4e83fb3e9151648d02011b7, ptr %_0, align 4, !dbg !1721
  %18 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1721
  store i32 11, ptr %18, align 4, !dbg !1721
  br label %bb79, !dbg !1721

bb60:                                             ; preds = %start
  store ptr @alloc_39ff86cee76cddbe698233f11e125157, ptr %_0, align 4, !dbg !1722
  %19 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1722
  store i32 3, ptr %19, align 4, !dbg !1722
  br label %bb79, !dbg !1722

bb59:                                             ; preds = %start
  store ptr @alloc_7532bc440e2a5cd291db16840aeb436b, ptr %_0, align 4, !dbg !1723
  %20 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1723
  store i32 5, ptr %20, align 4, !dbg !1723
  br label %bb79, !dbg !1723

bb58:                                             ; preds = %start
  store ptr @alloc_4b36768a1ccde10a4d74de32c7566dae, ptr %_0, align 4, !dbg !1724
  %21 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1724
  store i32 5, ptr %21, align 4, !dbg !1724
  br label %bb79, !dbg !1724

bb57:                                             ; preds = %start
  store ptr @alloc_f47d5f6d9505f19073a40da4f201e89e, ptr %_0, align 4, !dbg !1725
  %22 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1725
  store i32 5, ptr %22, align 4, !dbg !1725
  br label %bb79, !dbg !1725

bb56:                                             ; preds = %start
  store ptr @alloc_d2a082495eb1a324c2165af137b6713f, ptr %_0, align 4, !dbg !1726
  %23 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1726
  store i32 4, ptr %23, align 4, !dbg !1726
  br label %bb79, !dbg !1726

bb55:                                             ; preds = %start
  store ptr @alloc_e4472dd01e3df0559c103997859ede93, ptr %_0, align 4, !dbg !1727
  %24 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1727
  store i32 11, ptr %24, align 4, !dbg !1727
  br label %bb79, !dbg !1727

bb54:                                             ; preds = %start
  store ptr @alloc_820fc975c3e424e90b13ff5fd907362d, ptr %_0, align 4, !dbg !1728
  %25 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1728
  store i32 4, ptr %25, align 4, !dbg !1728
  br label %bb79, !dbg !1728

bb53:                                             ; preds = %start
  store ptr @alloc_8b2a908f67e94e7bbbaa8522b21febe6, ptr %_0, align 4, !dbg !1729
  %26 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1729
  store i32 5, ptr %26, align 4, !dbg !1729
  br label %bb79, !dbg !1729

bb52:                                             ; preds = %start
  store ptr @alloc_0e8392c5a03b89002a957a60556e615e, ptr %_0, align 4, !dbg !1730
  %27 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1730
  store i32 10, ptr %27, align 4, !dbg !1730
  br label %bb79, !dbg !1730

bb51:                                             ; preds = %start
  store ptr @alloc_d32891f9920c2db7cf73e87c7574e15e, ptr %_0, align 4, !dbg !1731
  %28 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1731
  store i32 4, ptr %28, align 4, !dbg !1731
  br label %bb79, !dbg !1731

bb50:                                             ; preds = %start
  store ptr @alloc_a5fe43a0c9a1b5894117e41abbec297f, ptr %_0, align 4, !dbg !1732
  %29 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1732
  store i32 5, ptr %29, align 4, !dbg !1732
  br label %bb79, !dbg !1732

bb49:                                             ; preds = %start
  store ptr @alloc_a9eaa09b855740b7afcf8c401274badc, ptr %_0, align 4, !dbg !1733
  %30 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1733
  store i32 2, ptr %30, align 4, !dbg !1733
  br label %bb79, !dbg !1733

bb48:                                             ; preds = %start
  store ptr @alloc_0a76b23954277669a40931fd8b7a3c0b, ptr %_0, align 4, !dbg !1734
  %31 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1734
  store i32 6, ptr %31, align 4, !dbg !1734
  br label %bb79, !dbg !1734

bb47:                                             ; preds = %start
  store ptr @alloc_48dd9fbc0aa0a1ec0bd01e76772188ca, ptr %_0, align 4, !dbg !1735
  %32 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1735
  store i32 5, ptr %32, align 4, !dbg !1735
  br label %bb79, !dbg !1735

bb46:                                             ; preds = %start
  store ptr @alloc_dd63b73adf6e9192eed7882c1e0dccda, ptr %_0, align 4, !dbg !1736
  %33 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1736
  store i32 4, ptr %33, align 4, !dbg !1736
  br label %bb79, !dbg !1736

bb45:                                             ; preds = %start
  store ptr @alloc_79ff279bd852f075db2122f2b968aef1, ptr %_0, align 4, !dbg !1737
  %34 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1737
  store i32 5, ptr %34, align 4, !dbg !1737
  br label %bb79, !dbg !1737

bb44:                                             ; preds = %start
  store ptr @alloc_7438b4b6de0425646bb9362129be2540, ptr %_0, align 4, !dbg !1738
  %35 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1738
  store i32 5, ptr %35, align 4, !dbg !1738
  br label %bb79, !dbg !1738

bb43:                                             ; preds = %start
  store ptr @alloc_08cd9c903b35ceb9437506b4b82226a7, ptr %_0, align 4, !dbg !1739
  %36 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1739
  store i32 7, ptr %36, align 4, !dbg !1739
  br label %bb79, !dbg !1739

bb42:                                             ; preds = %start
  store ptr @alloc_b67a0498ec853dd0b8fd56ca21603195, ptr %_0, align 4, !dbg !1740
  %37 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1740
  store i32 8, ptr %37, align 4, !dbg !1740
  br label %bb79, !dbg !1740

bb41:                                             ; preds = %start
  store ptr @alloc_8d9481f0445b1a3759b069a16ddbc6b8, ptr %_0, align 4, !dbg !1741
  %38 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1741
  store i32 11, ptr %38, align 4, !dbg !1741
  br label %bb79, !dbg !1741

bb40:                                             ; preds = %start
  store ptr @alloc_edaf563e7ced42fe9a4b29da8944b938, ptr %_0, align 4, !dbg !1742
  %39 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1742
  store i32 7, ptr %39, align 4, !dbg !1742
  br label %bb79, !dbg !1742

bb39:                                             ; preds = %start
  store ptr @alloc_33ba081697e0cbd20f7a3190e0e07ecd, ptr %_0, align 4, !dbg !1743
  %40 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1743
  store i32 8, ptr %40, align 4, !dbg !1743
  br label %bb79, !dbg !1743

bb38:                                             ; preds = %start
  store ptr @alloc_52700a7601ce3574eb8f6a19c2a2f3b0, ptr %_0, align 4, !dbg !1744
  %41 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1744
  store i32 10, ptr %41, align 4, !dbg !1744
  br label %bb79, !dbg !1744

bb37:                                             ; preds = %start
  store ptr @alloc_b6df84a30618b409bf4463f51730701e, ptr %_0, align 4, !dbg !1745
  %42 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1745
  store i32 5, ptr %42, align 4, !dbg !1745
  br label %bb79, !dbg !1745

bb36:                                             ; preds = %start
  store ptr @alloc_7ad907da16c51d1b1767f589dbedb81c, ptr %_0, align 4, !dbg !1746
  %43 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1746
  store i32 6, ptr %43, align 4, !dbg !1746
  br label %bb79, !dbg !1746

bb35:                                             ; preds = %start
  store ptr @alloc_390128a259ca122ccc7c101503dcf7e2, ptr %_0, align 4, !dbg !1747
  %44 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1747
  store i32 5, ptr %44, align 4, !dbg !1747
  br label %bb79, !dbg !1747

bb34:                                             ; preds = %start
  store ptr @alloc_77e32f5571452900dccaaddf2e771641, ptr %_0, align 4, !dbg !1748
  %45 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1748
  store i32 5, ptr %45, align 4, !dbg !1748
  br label %bb79, !dbg !1748

bb33:                                             ; preds = %start
  store ptr @alloc_672d3a2ca4a906d2c49343de26785b3d, ptr %_0, align 4, !dbg !1749
  %46 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1749
  store i32 6, ptr %46, align 4, !dbg !1749
  br label %bb79, !dbg !1749

bb32:                                             ; preds = %start
  store ptr @alloc_c2361c8fb3e1574e929b197bb6f642d9, ptr %_0, align 4, !dbg !1750
  %47 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1750
  store i32 5, ptr %47, align 4, !dbg !1750
  br label %bb79, !dbg !1750

bb31:                                             ; preds = %start
  store ptr @alloc_d34415974c96b1818039e07e24c9a542, ptr %_0, align 4, !dbg !1751
  %48 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1751
  store i32 6, ptr %48, align 4, !dbg !1751
  br label %bb79, !dbg !1751

bb30:                                             ; preds = %start
  store ptr @alloc_98446342a477dc4b70825be19fec689c, ptr %_0, align 4, !dbg !1752
  %49 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1752
  store i32 5, ptr %49, align 4, !dbg !1752
  br label %bb79, !dbg !1752

bb29:                                             ; preds = %start
  store ptr @alloc_9d29ccf1851f76dea2b1eda56b1339a4, ptr %_0, align 4, !dbg !1753
  %50 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1753
  store i32 5, ptr %50, align 4, !dbg !1753
  br label %bb79, !dbg !1753

bb28:                                             ; preds = %start
  store ptr @alloc_338071ff59f25595243757f423cf8c4d, ptr %_0, align 4, !dbg !1754
  %51 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1754
  store i32 10, ptr %51, align 4, !dbg !1754
  br label %bb79, !dbg !1754

bb27:                                             ; preds = %start
  store ptr @alloc_acb3dc91cb500f6df7d122691e000086, ptr %_0, align 4, !dbg !1755
  %52 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1755
  store i32 5, ptr %52, align 4, !dbg !1755
  br label %bb79, !dbg !1755

bb26:                                             ; preds = %start
  store ptr @alloc_0768e0e6701eea9bd662f098d8a76639, ptr %_0, align 4, !dbg !1756
  %53 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1756
  store i32 5, ptr %53, align 4, !dbg !1756
  br label %bb79, !dbg !1756

bb25:                                             ; preds = %start
  store ptr @alloc_aad7f4837a0f073d74c347d1aa856e5a, ptr %_0, align 4, !dbg !1757
  %54 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1757
  store i32 7, ptr %54, align 4, !dbg !1757
  br label %bb79, !dbg !1757

bb24:                                             ; preds = %start
  store ptr @alloc_cb80030390ab958e664277e7ca12fcff, ptr %_0, align 4, !dbg !1758
  %55 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1758
  store i32 6, ptr %55, align 4, !dbg !1758
  br label %bb79, !dbg !1758

bb23:                                             ; preds = %start
  store ptr @alloc_18d75c46ae0ab8f50e6836934fbf2600, ptr %_0, align 4, !dbg !1759
  %56 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1759
  store i32 8, ptr %56, align 4, !dbg !1759
  br label %bb79, !dbg !1759

bb22:                                             ; preds = %start
  store ptr @alloc_5daae7153a002e51f143b30b78309845, ptr %_0, align 4, !dbg !1760
  %57 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1760
  store i32 14, ptr %57, align 4, !dbg !1760
  br label %bb79, !dbg !1760

bb21:                                             ; preds = %start
  store ptr @alloc_1be84d0a1f1bf46ad4dc0650fdee1233, ptr %_0, align 4, !dbg !1761
  %58 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1761
  store i32 7, ptr %58, align 4, !dbg !1761
  br label %bb79, !dbg !1761

bb20:                                             ; preds = %start
  store ptr @alloc_fa922dc79f516041a49296256e336569, ptr %_0, align 4, !dbg !1762
  %59 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1762
  store i32 6, ptr %59, align 4, !dbg !1762
  br label %bb79, !dbg !1762

bb19:                                             ; preds = %start
  store ptr @alloc_35918dfdd30516e0576216d8bb37a7e8, ptr %_0, align 4, !dbg !1763
  %60 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1763
  store i32 5, ptr %60, align 4, !dbg !1763
  br label %bb79, !dbg !1763

bb18:                                             ; preds = %start
  store ptr @alloc_23e5ef0eb64cf9a4828eed9d2cdbb19f, ptr %_0, align 4, !dbg !1764
  %61 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1764
  store i32 4, ptr %61, align 4, !dbg !1764
  br label %bb79, !dbg !1764

bb17:                                             ; preds = %start
  store ptr @alloc_3b8b4793b1ea451345d29976e468891f, ptr %_0, align 4, !dbg !1765
  %62 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1765
  store i32 8, ptr %62, align 4, !dbg !1765
  br label %bb79, !dbg !1765

bb16:                                             ; preds = %start
  store ptr @alloc_a060d5bdeb7be0aa9e9bc429ea28a915, ptr %_0, align 4, !dbg !1766
  %63 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1766
  store i32 9, ptr %63, align 4, !dbg !1766
  br label %bb79, !dbg !1766

bb15:                                             ; preds = %start
  store ptr @alloc_c6663eba2feb4c0fa6a6601ac4c9f961, ptr %_0, align 4, !dbg !1767
  %64 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1767
  store i32 4, ptr %64, align 4, !dbg !1767
  br label %bb79, !dbg !1767

bb14:                                             ; preds = %start
  store ptr @alloc_4832908c0d36609818b44d52da3b4ab1, ptr %_0, align 4, !dbg !1768
  %65 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1768
  store i32 4, ptr %65, align 4, !dbg !1768
  br label %bb79, !dbg !1768

bb13:                                             ; preds = %start
  store ptr @alloc_39701fd8446760e713828a22e8e84657, ptr %_0, align 4, !dbg !1769
  %66 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1769
  store i32 5, ptr %66, align 4, !dbg !1769
  br label %bb79, !dbg !1769

bb12:                                             ; preds = %start
  store ptr @alloc_0fb76d18eb4a8293863d42d53c0164bc, ptr %_0, align 4, !dbg !1770
  %67 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1770
  store i32 14, ptr %67, align 4, !dbg !1770
  br label %bb79, !dbg !1770

bb11:                                             ; preds = %start
  store ptr @alloc_f0ee6b554743936719182439aea5a47f, ptr %_0, align 4, !dbg !1771
  %68 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1771
  store i32 9, ptr %68, align 4, !dbg !1771
  br label %bb79, !dbg !1771

bb10:                                             ; preds = %start
  store ptr @alloc_f1b4a2fb1dc3d6723575e33bd5591258, ptr %_0, align 4, !dbg !1772
  %69 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1772
  store i32 5, ptr %69, align 4, !dbg !1772
  br label %bb79, !dbg !1772

bb9:                                              ; preds = %start
  store ptr @alloc_1a181a492552b27e757c957fa98b4301, ptr %_0, align 4, !dbg !1773
  %70 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1773
  store i32 4, ptr %70, align 4, !dbg !1773
  br label %bb79, !dbg !1773

bb8:                                              ; preds = %start
  store ptr @alloc_f36662d78f18dcd86d6806ad83e3d270, ptr %_0, align 4, !dbg !1774
  %71 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1774
  store i32 5, ptr %71, align 4, !dbg !1774
  br label %bb79, !dbg !1774

bb7:                                              ; preds = %start
  store ptr @alloc_a512a652cf8a4f93826461fecbbde9de, ptr %_0, align 4, !dbg !1775
  %72 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1775
  store i32 4, ptr %72, align 4, !dbg !1775
  br label %bb79, !dbg !1775

bb6:                                              ; preds = %start
  store ptr @alloc_8118216a9976010c8fb26905b45815b0, ptr %_0, align 4, !dbg !1776
  %73 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1776
  store i32 5, ptr %73, align 4, !dbg !1776
  br label %bb79, !dbg !1776

bb5:                                              ; preds = %start
  store ptr @alloc_b79ec72229df8c2bb729b17b46549851, ptr %_0, align 4, !dbg !1777
  %74 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1777
  store i32 8, ptr %74, align 4, !dbg !1777
  br label %bb79, !dbg !1777

bb4:                                              ; preds = %start
  store ptr @alloc_3dbf8c65f045ddaf28c14ea28745ff8b, ptr %_0, align 4, !dbg !1778
  %75 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1778
  store i32 6, ptr %75, align 4, !dbg !1778
  br label %bb79, !dbg !1778

bb3:                                              ; preds = %start
  store ptr @alloc_cbc52d0443417071f78111a521226f88, ptr %_0, align 4, !dbg !1779
  %76 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1779
  store i32 4, ptr %76, align 4, !dbg !1779
  br label %bb79, !dbg !1779

bb2:                                              ; preds = %start
  store ptr @alloc_98b30359d473d22b9e60dcaa28fd9cc0, ptr %_0, align 4, !dbg !1780
  %77 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1780
  store i32 10, ptr %77, align 4, !dbg !1780
  br label %bb79, !dbg !1780

bb79:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23, %bb24, %bb25, %bb26, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32, %bb33, %bb34, %bb35, %bb36, %bb37, %bb38, %bb39, %bb40, %bb41, %bb42, %bb43, %bb44, %bb45, %bb46, %bb47, %bb48, %bb49, %bb50, %bb51, %bb52, %bb53, %bb54, %bb55, %bb56, %bb57, %bb58, %bb59, %bb60, %bb61, %bb62, %bb63, %bb64, %bb65, %bb66, %bb67, %bb68, %bb69, %bb70, %bb71, %bb72, %bb73, %bb74, %bb75, %bb76, %bb77, %bb78
  %78 = load ptr, ptr %_0, align 4, !dbg !1781
  %79 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1781
  %80 = load i32, ptr %79, align 4, !dbg !1781
  %81 = insertvalue { ptr, i32 } poison, ptr %78, 0, !dbg !1781
  %82 = insertvalue { ptr, i32 } %81, i32 %80, 1, !dbg !1781
  ret { ptr, i32 } %82, !dbg !1781
}

; wasi::lib_generated::Errno::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated5Errno7message17h331e22806c2b66acE(ptr align 2 %self) unnamed_addr #2 !dbg !1782 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1785, !DIExpression(), !1786)
  %0 = load i16, ptr %self, align 2, !dbg !1787
  switch i16 %0, label %bb1 [
    i16 0, label %bb78
    i16 1, label %bb77
    i16 2, label %bb76
    i16 3, label %bb75
    i16 4, label %bb74
    i16 5, label %bb73
    i16 6, label %bb72
    i16 7, label %bb71
    i16 8, label %bb70
    i16 9, label %bb69
    i16 10, label %bb68
    i16 11, label %bb67
    i16 12, label %bb66
    i16 13, label %bb65
    i16 14, label %bb64
    i16 15, label %bb63
    i16 16, label %bb62
    i16 17, label %bb61
    i16 18, label %bb60
    i16 19, label %bb59
    i16 20, label %bb58
    i16 21, label %bb57
    i16 22, label %bb56
    i16 23, label %bb55
    i16 24, label %bb54
    i16 25, label %bb53
    i16 26, label %bb52
    i16 27, label %bb51
    i16 28, label %bb50
    i16 29, label %bb49
    i16 30, label %bb48
    i16 31, label %bb47
    i16 32, label %bb46
    i16 33, label %bb45
    i16 34, label %bb44
    i16 35, label %bb43
    i16 36, label %bb42
    i16 37, label %bb41
    i16 38, label %bb40
    i16 39, label %bb39
    i16 40, label %bb38
    i16 41, label %bb37
    i16 42, label %bb36
    i16 43, label %bb35
    i16 44, label %bb34
    i16 45, label %bb33
    i16 46, label %bb32
    i16 47, label %bb31
    i16 48, label %bb30
    i16 49, label %bb29
    i16 50, label %bb28
    i16 51, label %bb27
    i16 52, label %bb26
    i16 53, label %bb25
    i16 54, label %bb24
    i16 55, label %bb23
    i16 56, label %bb22
    i16 57, label %bb21
    i16 58, label %bb20
    i16 59, label %bb19
    i16 60, label %bb18
    i16 61, label %bb17
    i16 62, label %bb16
    i16 63, label %bb15
    i16 64, label %bb14
    i16 65, label %bb13
    i16 66, label %bb12
    i16 67, label %bb11
    i16 68, label %bb10
    i16 69, label %bb9
    i16 70, label %bb8
    i16 71, label %bb7
    i16 72, label %bb6
    i16 73, label %bb5
    i16 74, label %bb4
    i16 75, label %bb3
    i16 76, label %bb2
  ], !dbg !1787

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_92339703776781697a7015ea2de787b5) #53, !dbg !1788
  unreachable, !dbg !1788

bb78:                                             ; preds = %start
  store ptr @alloc_738b72084b53a4d34e6543f6fa0616e5, ptr %_0, align 4, !dbg !1789
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1789
  store i32 54, ptr %1, align 4, !dbg !1789
  br label %bb79, !dbg !1789

bb77:                                             ; preds = %start
  store ptr @alloc_63cb38bf054ed0aa1c1989dca559a70d, ptr %_0, align 4, !dbg !1790
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1790
  store i32 23, ptr %2, align 4, !dbg !1790
  br label %bb79, !dbg !1790

bb76:                                             ; preds = %start
  store ptr @alloc_6f0e46a1b1c28b0b5efd847e7c5099af, ptr %_0, align 4, !dbg !1791
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1791
  store i32 18, ptr %3, align 4, !dbg !1791
  br label %bb79, !dbg !1791

bb75:                                             ; preds = %start
  store ptr @alloc_6336caef1304c9177e345ebb2e043fda, ptr %_0, align 4, !dbg !1792
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1792
  store i32 15, ptr %4, align 4, !dbg !1792
  br label %bb79, !dbg !1792

bb74:                                             ; preds = %start
  store ptr @alloc_019b84fb7e896b1e2ff05536dcbff498, ptr %_0, align 4, !dbg !1793
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1793
  store i32 22, ptr %5, align 4, !dbg !1793
  br label %bb79, !dbg !1793

bb73:                                             ; preds = %start
  store ptr @alloc_41a7c3ef62d536421e93c122f5fe7b8d, ptr %_0, align 4, !dbg !1794
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1794
  store i32 29, ptr %6, align 4, !dbg !1794
  br label %bb79, !dbg !1794

bb72:                                             ; preds = %start
  store ptr @alloc_72667a1889bb1fb3adcae40a49439f4e, ptr %_0, align 4, !dbg !1795
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1795
  store i32 47, ptr %7, align 4, !dbg !1795
  br label %bb79, !dbg !1795

bb71:                                             ; preds = %start
  store ptr @alloc_c75ac68611722b9931e3c07c7d349ff9, ptr %_0, align 4, !dbg !1796
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1796
  store i32 31, ptr %8, align 4, !dbg !1796
  br label %bb79, !dbg !1796

bb70:                                             ; preds = %start
  store ptr @alloc_50bf1fbfc329f4125b504796273167be, ptr %_0, align 4, !dbg !1797
  %9 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1797
  store i32 20, ptr %9, align 4, !dbg !1797
  br label %bb79, !dbg !1797

bb69:                                             ; preds = %start
  store ptr @alloc_db7708960a5e94965c9dbdd9624fc4cb, ptr %_0, align 4, !dbg !1798
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1798
  store i32 12, ptr %10, align 4, !dbg !1798
  br label %bb79, !dbg !1798

bb68:                                             ; preds = %start
  store ptr @alloc_f0bed62e43f778e5a8496b2c99a845d7, ptr %_0, align 4, !dbg !1799
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1799
  store i32 24, ptr %11, align 4, !dbg !1799
  br label %bb79, !dbg !1799

bb67:                                             ; preds = %start
  store ptr @alloc_c4e6fe31be9e1422f6c2c65a1d48a6b3, ptr %_0, align 4, !dbg !1800
  %12 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1800
  store i32 19, ptr %12, align 4, !dbg !1800
  br label %bb79, !dbg !1800

bb66:                                             ; preds = %start
  store ptr @alloc_d14052a2d17b8b4721b366ca99170a7e, ptr %_0, align 4, !dbg !1801
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1801
  store i32 19, ptr %13, align 4, !dbg !1801
  br label %bb79, !dbg !1801

bb65:                                             ; preds = %start
  store ptr @alloc_6cde56766d930cfca05b143c24c97a3c, ptr %_0, align 4, !dbg !1802
  %14 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1802
  store i32 19, ptr %14, align 4, !dbg !1802
  br label %bb79, !dbg !1802

bb64:                                             ; preds = %start
  store ptr @alloc_f576af0d4e37b0b54eba3c15d93ddbd5, ptr %_0, align 4, !dbg !1803
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1803
  store i32 19, ptr %15, align 4, !dbg !1803
  br label %bb79, !dbg !1803

bb63:                                             ; preds = %start
  store ptr @alloc_69a56768e5c93b3bcd705dc2bbf127c1, ptr %_0, align 4, !dbg !1804
  %16 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1804
  store i32 17, ptr %16, align 4, !dbg !1804
  br label %bb79, !dbg !1804

bb62:                                             ; preds = %start
  store ptr @alloc_596999da3cde17096c9e2140c0918ba4, ptr %_0, align 4, !dbg !1805
  %17 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1805
  store i32 30, ptr %17, align 4, !dbg !1805
  br label %bb79, !dbg !1805

bb61:                                             ; preds = %start
  store ptr @alloc_e25cb539fdcd1152765745e7db037191, ptr %_0, align 4, !dbg !1806
  %18 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1806
  store i32 29, ptr %18, align 4, !dbg !1806
  br label %bb79, !dbg !1806

bb60:                                             ; preds = %start
  store ptr @alloc_f457658de3d1562f99548ba5528655f8, ptr %_0, align 4, !dbg !1807
  %19 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1807
  store i32 47, ptr %19, align 4, !dbg !1807
  br label %bb79, !dbg !1807

bb59:                                             ; preds = %start
  store ptr @alloc_d8f7503a36cdc9bc6346c9e92a42bc81, ptr %_0, align 4, !dbg !1808
  %20 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1808
  store i32 9, ptr %20, align 4, !dbg !1808
  br label %bb79, !dbg !1808

bb58:                                             ; preds = %start
  store ptr @alloc_f12c98ce38519e21b9c9da75f9f4fc43, ptr %_0, align 4, !dbg !1809
  %21 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1809
  store i32 12, ptr %21, align 4, !dbg !1809
  br label %bb79, !dbg !1809

bb57:                                             ; preds = %start
  store ptr @alloc_a91e11b0ce836ee177ea52c8e7cd8d50, ptr %_0, align 4, !dbg !1810
  %22 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1810
  store i32 12, ptr %22, align 4, !dbg !1810
  br label %bb79, !dbg !1810

bb56:                                             ; preds = %start
  store ptr @alloc_c41cb924416df4aca21adddaa5bd21dd, ptr %_0, align 4, !dbg !1811
  %23 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1811
  store i32 15, ptr %23, align 4, !dbg !1811
  br label %bb79, !dbg !1811

bb55:                                             ; preds = %start
  store ptr @alloc_dfb69db467b1a3656fb6fdbace99c9b9, ptr %_0, align 4, !dbg !1812
  %24 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1812
  store i32 20, ptr %24, align 4, !dbg !1812
  br label %bb79, !dbg !1812

bb54:                                             ; preds = %start
  store ptr @alloc_9b981c3df33594e259c917c223bfc7a6, ptr %_0, align 4, !dbg !1813
  %25 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1813
  store i32 19, ptr %25, align 4, !dbg !1813
  br label %bb79, !dbg !1813

bb53:                                             ; preds = %start
  store ptr @alloc_4bfe52500497b30818b54150a30d4f08, ptr %_0, align 4, !dbg !1814
  %26 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1814
  store i32 22, ptr %26, align 4, !dbg !1814
  br label %bb79, !dbg !1814

bb52:                                             ; preds = %start
  store ptr @alloc_4f6557f2690f452d2e5554006942347f, ptr %_0, align 4, !dbg !1815
  %27 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1815
  store i32 22, ptr %27, align 4, !dbg !1815
  br label %bb79, !dbg !1815

bb51:                                             ; preds = %start
  store ptr @alloc_d073b4ff65817f21be5ef28dd0a03cda, ptr %_0, align 4, !dbg !1816
  %28 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1816
  store i32 21, ptr %28, align 4, !dbg !1816
  br label %bb79, !dbg !1816

bb50:                                             ; preds = %start
  store ptr @alloc_2d0f752c50db342d0d94862ec0dd55e9, ptr %_0, align 4, !dbg !1817
  %29 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1817
  store i32 17, ptr %29, align 4, !dbg !1817
  br label %bb79, !dbg !1817

bb49:                                             ; preds = %start
  store ptr @alloc_38bacb6f728efb49beca495dbc067ccf, ptr %_0, align 4, !dbg !1818
  %30 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1818
  store i32 10, ptr %30, align 4, !dbg !1818
  br label %bb79, !dbg !1818

bb48:                                             ; preds = %start
  store ptr @alloc_c0362ec6ca12ceceb48261de984a7166, ptr %_0, align 4, !dbg !1819
  %31 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1819
  store i32 20, ptr %31, align 4, !dbg !1819
  br label %bb79, !dbg !1819

bb47:                                             ; preds = %start
  store ptr @alloc_31b18777694e6f001b4e1d2203fbab80, ptr %_0, align 4, !dbg !1820
  %32 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1820
  store i32 15, ptr %32, align 4, !dbg !1820
  br label %bb79, !dbg !1820

bb46:                                             ; preds = %start
  store ptr @alloc_bd5d88c6439a3c98342310082da1f1c2, ptr %_0, align 4, !dbg !1821
  %33 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1821
  store i32 34, ptr %33, align 4, !dbg !1821
  br label %bb79, !dbg !1821

bb45:                                             ; preds = %start
  store ptr @alloc_9b00bf851dceae7b81b26420d02dd5f2, ptr %_0, align 4, !dbg !1822
  %34 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1822
  store i32 32, ptr %34, align 4, !dbg !1822
  br label %bb79, !dbg !1822

bb44:                                             ; preds = %start
  store ptr @alloc_83d8f44faafe130e1396daa0c5b148d0, ptr %_0, align 4, !dbg !1823
  %35 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1823
  store i32 15, ptr %35, align 4, !dbg !1823
  br label %bb79, !dbg !1823

bb43:                                             ; preds = %start
  store ptr @alloc_26338639bc7c94a746aa95f550572308, ptr %_0, align 4, !dbg !1824
  %36 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1824
  store i32 18, ptr %36, align 4, !dbg !1824
  br label %bb79, !dbg !1824

bb42:                                             ; preds = %start
  store ptr @alloc_d8f7503a36cdc9bc6346c9e92a42bc81, ptr %_0, align 4, !dbg !1825
  %37 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1825
  store i32 9, ptr %37, align 4, !dbg !1825
  br label %bb79, !dbg !1825

bb41:                                             ; preds = %start
  store ptr @alloc_1f1653a1bd7a0c91ce65b8bdfb6e14d3, ptr %_0, align 4, !dbg !1826
  %38 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1826
  store i32 18, ptr %38, align 4, !dbg !1826
  br label %bb79, !dbg !1826

bb40:                                             ; preds = %start
  store ptr @alloc_5aea10ec52d9b12f8025d677170427ae, ptr %_0, align 4, !dbg !1827
  %39 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1827
  store i32 16, ptr %39, align 4, !dbg !1827
  br label %bb79, !dbg !1827

bb39:                                             ; preds = %start
  store ptr @alloc_4737f61e08be7fd4781363eaf0ba41f3, ptr %_0, align 4, !dbg !1828
  %40 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1828
  store i32 30, ptr %40, align 4, !dbg !1828
  br label %bb79, !dbg !1828

bb38:                                             ; preds = %start
  store ptr @alloc_42fe82e120ce05d367941be8c45758d6, ptr %_0, align 4, !dbg !1829
  %41 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1829
  store i32 20, ptr %41, align 4, !dbg !1829
  br label %bb79, !dbg !1829

bb37:                                             ; preds = %start
  store ptr @alloc_210a08b40d17b5afdf1083a0818a09c2, ptr %_0, align 4, !dbg !1830
  %42 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1830
  store i32 30, ptr %42, align 4, !dbg !1830
  br label %bb79, !dbg !1830

bb36:                                             ; preds = %start
  store ptr @alloc_ea505343c96a58ff92689d83b2ef7605, ptr %_0, align 4, !dbg !1831
  %43 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1831
  store i32 26, ptr %43, align 4, !dbg !1831
  br label %bb79, !dbg !1831

bb35:                                             ; preds = %start
  store ptr @alloc_f8ce4cd55bfbd4ae871da2f1a4336f45, ptr %_0, align 4, !dbg !1832
  %44 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1832
  store i32 15, ptr %44, align 4, !dbg !1832
  br label %bb79, !dbg !1832

bb34:                                             ; preds = %start
  store ptr @alloc_aff65f0f6f2c87a1347784f12b25642c, ptr %_0, align 4, !dbg !1833
  %45 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1833
  store i32 26, ptr %45, align 4, !dbg !1833
  br label %bb79, !dbg !1833

bb33:                                             ; preds = %start
  store ptr @alloc_bcc1ecb9294c681d33e8c041b1bcbfbf, ptr %_0, align 4, !dbg !1834
  %46 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1834
  store i32 29, ptr %46, align 4, !dbg !1834
  br label %bb79, !dbg !1834

bb32:                                             ; preds = %start
  store ptr @alloc_2c388975459832c1da18b16bb6f6cad7, ptr %_0, align 4, !dbg !1835
  %47 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1835
  store i32 19, ptr %47, align 4, !dbg !1835
  br label %bb79, !dbg !1835

bb31:                                             ; preds = %start
  store ptr @alloc_d8f7503a36cdc9bc6346c9e92a42bc81, ptr %_0, align 4, !dbg !1836
  %48 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1836
  store i32 9, ptr %48, align 4, !dbg !1836
  br label %bb79, !dbg !1836

bb30:                                             ; preds = %start
  store ptr @alloc_9cc3052ce766bc8d1bd2f0472f25d6ed, ptr %_0, align 4, !dbg !1837
  %49 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1837
  store i32 17, ptr %49, align 4, !dbg !1837
  br label %bb79, !dbg !1837

bb29:                                             ; preds = %start
  store ptr @alloc_c5d465d10be95607dc340c7a4d24f2bd, ptr %_0, align 4, !dbg !1838
  %50 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1838
  store i32 31, ptr %50, align 4, !dbg !1838
  br label %bb79, !dbg !1838

bb28:                                             ; preds = %start
  store ptr @alloc_6ceecfbc50ac588c65f787ca7566721e, ptr %_0, align 4, !dbg !1839
  %51 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1839
  store i32 23, ptr %51, align 4, !dbg !1839
  br label %bb79, !dbg !1839

bb27:                                             ; preds = %start
  store ptr @alloc_ad22dde9eb788be408dfbce5b536ecb1, ptr %_0, align 4, !dbg !1840
  %52 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1840
  store i32 24, ptr %52, align 4, !dbg !1840
  br label %bb79, !dbg !1840

bb26:                                             ; preds = %start
  store ptr @alloc_a5c36ad2f7f39f8c571bed384c4aa86e, ptr %_0, align 4, !dbg !1841
  %53 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1841
  store i32 23, ptr %53, align 4, !dbg !1841
  br label %bb79, !dbg !1841

bb25:                                             ; preds = %start
  store ptr @alloc_79e5afbb2836e42f1cf6545735def723, ptr %_0, align 4, !dbg !1842
  %54 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1842
  store i32 28, ptr %54, align 4, !dbg !1842
  br label %bb79, !dbg !1842

bb24:                                             ; preds = %start
  store ptr @alloc_b79b88e1517127bd26a11caf03e96a4b, ptr %_0, align 4, !dbg !1843
  %55 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1843
  store i32 50, ptr %55, align 4, !dbg !1843
  br label %bb79, !dbg !1843

bb23:                                             ; preds = %start
  store ptr @alloc_5bd796a2729b29dc1077aa78010540eb, ptr %_0, align 4, !dbg !1844
  %56 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1844
  store i32 20, ptr %56, align 4, !dbg !1844
  br label %bb79, !dbg !1844

bb22:                                             ; preds = %start
  store ptr @alloc_2227009005ed3aef895a625a934fb490, ptr %_0, align 4, !dbg !1845
  %57 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1845
  store i32 22, ptr %57, align 4, !dbg !1845
  br label %bb79, !dbg !1845

bb21:                                             ; preds = %start
  store ptr @alloc_7b2218a5156569098ceb294456ea6101, ptr %_0, align 4, !dbg !1846
  %58 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1846
  store i32 13, ptr %58, align 4, !dbg !1846
  br label %bb79, !dbg !1846

bb20:                                             ; preds = %start
  store ptr @alloc_ee0d04358a5a06f6330667e5324932c2, ptr %_0, align 4, !dbg !1847
  %59 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1847
  store i32 52, ptr %59, align 4, !dbg !1847
  br label %bb79, !dbg !1847

bb19:                                             ; preds = %start
  store ptr @alloc_e6d870eee4288e15aaacc1f81fe3252c, ptr %_0, align 4, !dbg !1848
  %60 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1848
  store i32 36, ptr %60, align 4, !dbg !1848
  br label %bb79, !dbg !1848

bb18:                                             ; preds = %start
  store ptr @alloc_b33f52b7e7f121d0c226151f5c778cf3, ptr %_0, align 4, !dbg !1849
  %61 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1849
  store i32 26, ptr %61, align 4, !dbg !1849
  br label %bb79, !dbg !1849

bb17:                                             ; preds = %start
  store ptr @alloc_5ae69e8e195aceb61f3f3c18eb8bcf47, ptr %_0, align 4, !dbg !1850
  %62 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1850
  store i32 42, ptr %62, align 4, !dbg !1850
  br label %bb79, !dbg !1850

bb16:                                             ; preds = %start
  store ptr @alloc_2ea23f84b1b3d5f1a01ca6aee02ddfe3, ptr %_0, align 4, !dbg !1851
  %63 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1851
  store i32 20, ptr %63, align 4, !dbg !1851
  br label %bb79, !dbg !1851

bb15:                                             ; preds = %start
  store ptr @alloc_7541958cba33228c76ccfd6ef3a998f8, ptr %_0, align 4, !dbg !1852
  %64 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1852
  store i32 24, ptr %64, align 4, !dbg !1852
  br label %bb79, !dbg !1852

bb14:                                             ; preds = %start
  store ptr @alloc_267ce7f25696333bd0d66667a08bb4cc, ptr %_0, align 4, !dbg !1853
  %65 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1853
  store i32 12, ptr %65, align 4, !dbg !1853
  br label %bb79, !dbg !1853

bb13:                                             ; preds = %start
  store ptr @alloc_651051b66cd9fca775b419eff6fac487, ptr %_0, align 4, !dbg !1854
  %66 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1854
  store i32 15, ptr %66, align 4, !dbg !1854
  br label %bb79, !dbg !1854

bb12:                                             ; preds = %start
  store ptr @alloc_34caa06c16574cedbfc891de933cd74b, ptr %_0, align 4, !dbg !1855
  %67 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1855
  store i32 23, ptr %67, align 4, !dbg !1855
  br label %bb79, !dbg !1855

bb11:                                             ; preds = %start
  store ptr @alloc_e4d49fcb16c811e138f85e7abb11a382, ptr %_0, align 4, !dbg !1856
  %68 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1856
  store i32 31, ptr %68, align 4, !dbg !1856
  br label %bb79, !dbg !1856

bb10:                                             ; preds = %start
  store ptr @alloc_05506144d01cb8b7afa4dd1845c86028, ptr %_0, align 4, !dbg !1857
  %69 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1857
  store i32 17, ptr %69, align 4, !dbg !1857
  br label %bb79, !dbg !1857

bb9:                                              ; preds = %start
  store ptr @alloc_802a116d829d4cbb92a657c6226a6057, ptr %_0, align 4, !dbg !1858
  %70 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1858
  store i32 22, ptr %70, align 4, !dbg !1858
  br label %bb79, !dbg !1858

bb8:                                              ; preds = %start
  store ptr @alloc_b6a7d25ded2c9ed637d0c584657272ef, ptr %_0, align 4, !dbg !1859
  %71 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1859
  store i32 13, ptr %71, align 4, !dbg !1859
  br label %bb79, !dbg !1859

bb7:                                              ; preds = %start
  store ptr @alloc_40faf286812d0e936efdc3d065b7fc25, ptr %_0, align 4, !dbg !1860
  %72 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1860
  store i32 16, ptr %72, align 4, !dbg !1860
  br label %bb79, !dbg !1860

bb6:                                              ; preds = %start
  store ptr @alloc_d8f7503a36cdc9bc6346c9e92a42bc81, ptr %_0, align 4, !dbg !1861
  %73 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1861
  store i32 9, ptr %73, align 4, !dbg !1861
  br label %bb79, !dbg !1861

bb5:                                              ; preds = %start
  store ptr @alloc_39b15d988456c8fcdc341a0d6e07d435, ptr %_0, align 4, !dbg !1862
  %74 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1862
  store i32 21, ptr %74, align 4, !dbg !1862
  br label %bb79, !dbg !1862

bb4:                                              ; preds = %start
  store ptr @alloc_095b6ceaeb015f29eb0c009fbd12eb42, ptr %_0, align 4, !dbg !1863
  %75 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1863
  store i32 15, ptr %75, align 4, !dbg !1863
  br label %bb79, !dbg !1863

bb3:                                              ; preds = %start
  store ptr @alloc_6dbbe3e0d2d606ce792f0527a3d75325, ptr %_0, align 4, !dbg !1864
  %76 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1864
  store i32 18, ptr %76, align 4, !dbg !1864
  br label %bb79, !dbg !1864

bb2:                                              ; preds = %start
  store ptr @alloc_090e2e52fa078921cfe166d26b6497f7, ptr %_0, align 4, !dbg !1865
  %77 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1865
  store i32 37, ptr %77, align 4, !dbg !1865
  br label %bb79, !dbg !1865

bb79:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23, %bb24, %bb25, %bb26, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32, %bb33, %bb34, %bb35, %bb36, %bb37, %bb38, %bb39, %bb40, %bb41, %bb42, %bb43, %bb44, %bb45, %bb46, %bb47, %bb48, %bb49, %bb50, %bb51, %bb52, %bb53, %bb54, %bb55, %bb56, %bb57, %bb58, %bb59, %bb60, %bb61, %bb62, %bb63, %bb64, %bb65, %bb66, %bb67, %bb68, %bb69, %bb70, %bb71, %bb72, %bb73, %bb74, %bb75, %bb76, %bb77, %bb78
  %78 = load ptr, ptr %_0, align 4, !dbg !1866
  %79 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1866
  %80 = load i32, ptr %79, align 4, !dbg !1866
  %81 = insertvalue { ptr, i32 } poison, ptr %78, 0, !dbg !1866
  %82 = insertvalue { ptr, i32 } %81, i32 %80, 1, !dbg !1866
  ret { ptr, i32 } %82, !dbg !1866
}

; wasi::lib_generated::Advice::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated6Advice3raw17h15896fee8c180decE(ptr align 1 %self) unnamed_addr #2 !dbg !1867 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1876, !DIExpression(), !1877)
  %_0 = load i8, ptr %self, align 1, !dbg !1878
  ret i8 %_0, !dbg !1879
}

; wasi::lib_generated::Advice::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Advice4name17h144c54f5ff34b5e2E(ptr align 1 %self) unnamed_addr #2 !dbg !1880 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1885, !DIExpression(), !1886)
  %0 = load i8, ptr %self, align 1, !dbg !1887
  switch i8 %0, label %bb1 [
    i8 0, label %bb7
    i8 1, label %bb6
    i8 2, label %bb5
    i8 3, label %bb4
    i8 4, label %bb3
    i8 5, label %bb2
  ], !dbg !1887

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_0bce313c2a20451cabb54b4d3072bba5) #53, !dbg !1888
  unreachable, !dbg !1888

bb7:                                              ; preds = %start
  store ptr @alloc_3eb08fd558da99ff59a728fa22216608, ptr %_0, align 4, !dbg !1889
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1889
  store i32 6, ptr %1, align 4, !dbg !1889
  br label %bb8, !dbg !1889

bb6:                                              ; preds = %start
  store ptr @alloc_c6c5eb2716d59a1e76b4d880a95c1203, ptr %_0, align 4, !dbg !1890
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1890
  store i32 10, ptr %2, align 4, !dbg !1890
  br label %bb8, !dbg !1890

bb5:                                              ; preds = %start
  store ptr @alloc_fc1d752d3184d907ca6eeb15408fb425, ptr %_0, align 4, !dbg !1891
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1891
  store i32 6, ptr %3, align 4, !dbg !1891
  br label %bb8, !dbg !1891

bb4:                                              ; preds = %start
  store ptr @alloc_65a421ac3660e213e6276b3729b0fb05, ptr %_0, align 4, !dbg !1892
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1892
  store i32 8, ptr %4, align 4, !dbg !1892
  br label %bb8, !dbg !1892

bb3:                                              ; preds = %start
  store ptr @alloc_2b00a549833884ad7baf6eabe3256f4c, ptr %_0, align 4, !dbg !1893
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1893
  store i32 8, ptr %5, align 4, !dbg !1893
  br label %bb8, !dbg !1893

bb2:                                              ; preds = %start
  store ptr @alloc_9ee9b0fb51e102c8b3d02ce7e5369dcf, ptr %_0, align 4, !dbg !1894
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1894
  store i32 7, ptr %6, align 4, !dbg !1894
  br label %bb8, !dbg !1894

bb8:                                              ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7
  %7 = load ptr, ptr %_0, align 4, !dbg !1895
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1895
  %9 = load i32, ptr %8, align 4, !dbg !1895
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0, !dbg !1895
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1, !dbg !1895
  ret { ptr, i32 } %11, !dbg !1895
}

; wasi::lib_generated::Advice::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Advice7message17h7a17fd969790b8c7E(ptr align 1 %self) unnamed_addr #2 !dbg !1896 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1899, !DIExpression(), !1900)
  %0 = load i8, ptr %self, align 1, !dbg !1901
  switch i8 %0, label %bb1 [
    i8 0, label %bb7
    i8 1, label %bb6
    i8 2, label %bb5
    i8 3, label %bb4
    i8 4, label %bb3
    i8 5, label %bb2
  ], !dbg !1901

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_28e1352c6e0fbf0afa06b76f00de61e7) #53, !dbg !1902
  unreachable, !dbg !1902

bb7:                                              ; preds = %start
  store ptr @alloc_2f1b7feb59b741ed0dcaa6768be38f92, ptr %_0, align 4, !dbg !1903
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1903
  store i32 89, ptr %1, align 4, !dbg !1903
  br label %bb8, !dbg !1903

bb6:                                              ; preds = %start
  store ptr @alloc_cae59b4057e77128fda7a33586d57565, ptr %_0, align 4, !dbg !1904
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1904
  store i32 103, ptr %2, align 4, !dbg !1904
  br label %bb8, !dbg !1904

bb5:                                              ; preds = %start
  store ptr @alloc_7379a0ec492be39e3d90d8d9194bce4b, ptr %_0, align 4, !dbg !1905
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1905
  store i32 71, ptr %3, align 4, !dbg !1905
  br label %bb8, !dbg !1905

bb4:                                              ; preds = %start
  store ptr @alloc_46b81c5cab4937138c3046baf3ce0eaa, ptr %_0, align 4, !dbg !1906
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1906
  store i32 72, ptr %4, align 4, !dbg !1906
  br label %bb8, !dbg !1906

bb3:                                              ; preds = %start
  store ptr @alloc_c98966898364094a7765a5dee620d7b7, ptr %_0, align 4, !dbg !1907
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1907
  store i32 86, ptr %5, align 4, !dbg !1907
  br label %bb8, !dbg !1907

bb2:                                              ; preds = %start
  store ptr @alloc_33a9062b2e552fbd5532b3c7655622a8, ptr %_0, align 4, !dbg !1908
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1908
  store i32 91, ptr %6, align 4, !dbg !1908
  br label %bb8, !dbg !1908

bb8:                                              ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7
  %7 = load ptr, ptr %_0, align 4, !dbg !1909
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1909
  %9 = load i32, ptr %8, align 4, !dbg !1909
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0, !dbg !1909
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1, !dbg !1909
  ret { ptr, i32 } %11, !dbg !1909
}

; wasi::lib_generated::Signal::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated6Signal3raw17hf6c9fc427e6cf57fE(ptr align 1 %self) unnamed_addr #2 !dbg !1910 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1916, !DIExpression(), !1917)
  %_0 = load i8, ptr %self, align 1, !dbg !1918
  ret i8 %_0, !dbg !1919
}

; wasi::lib_generated::Signal::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Signal4name17h44b7507ff21226fcE(ptr align 1 %self) unnamed_addr #2 !dbg !1920 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1925, !DIExpression(), !1926)
  %0 = load i8, ptr %self, align 1, !dbg !1927
  switch i8 %0, label %bb1 [
    i8 0, label %bb32
    i8 1, label %bb31
    i8 2, label %bb30
    i8 3, label %bb29
    i8 4, label %bb28
    i8 5, label %bb27
    i8 6, label %bb26
    i8 7, label %bb25
    i8 8, label %bb24
    i8 9, label %bb23
    i8 10, label %bb22
    i8 11, label %bb21
    i8 12, label %bb20
    i8 13, label %bb19
    i8 14, label %bb18
    i8 15, label %bb17
    i8 16, label %bb16
    i8 17, label %bb15
    i8 18, label %bb14
    i8 19, label %bb13
    i8 20, label %bb12
    i8 21, label %bb11
    i8 22, label %bb10
    i8 23, label %bb9
    i8 24, label %bb8
    i8 25, label %bb7
    i8 26, label %bb6
    i8 27, label %bb5
    i8 28, label %bb4
    i8 29, label %bb3
    i8 30, label %bb2
  ], !dbg !1927

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_26b978e47b28a070f1ba411c19d37f13) #53, !dbg !1928
  unreachable, !dbg !1928

bb32:                                             ; preds = %start
  store ptr @alloc_7b2f6f7016e80840b06d869ab1ff6b9c, ptr %_0, align 4, !dbg !1929
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1929
  store i32 4, ptr %1, align 4, !dbg !1929
  br label %bb33, !dbg !1929

bb31:                                             ; preds = %start
  store ptr @alloc_5305d1a6cfb98272599f584034cb1d99, ptr %_0, align 4, !dbg !1930
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1930
  store i32 3, ptr %2, align 4, !dbg !1930
  br label %bb33, !dbg !1930

bb30:                                             ; preds = %start
  store ptr @alloc_9ecfdfa2c73d42c419fcbbe6557cd56c, ptr %_0, align 4, !dbg !1931
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1931
  store i32 3, ptr %3, align 4, !dbg !1931
  br label %bb33, !dbg !1931

bb29:                                             ; preds = %start
  store ptr @alloc_006b3ed861a34a316b91090b379496e8, ptr %_0, align 4, !dbg !1932
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1932
  store i32 4, ptr %4, align 4, !dbg !1932
  br label %bb33, !dbg !1932

bb28:                                             ; preds = %start
  store ptr @alloc_2c150e288be0a9bb436351adfd41c311, ptr %_0, align 4, !dbg !1933
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1933
  store i32 3, ptr %5, align 4, !dbg !1933
  br label %bb33, !dbg !1933

bb27:                                             ; preds = %start
  store ptr @alloc_e01af9736b524006e5fb45ad54018c38, ptr %_0, align 4, !dbg !1934
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1934
  store i32 4, ptr %6, align 4, !dbg !1934
  br label %bb33, !dbg !1934

bb26:                                             ; preds = %start
  store ptr @alloc_d347cac5445f7cb2d7b668ea9b684875, ptr %_0, align 4, !dbg !1935
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1935
  store i32 4, ptr %7, align 4, !dbg !1935
  br label %bb33, !dbg !1935

bb25:                                             ; preds = %start
  store ptr @alloc_a467c9853a95bc3886e7dda7c3f32aae, ptr %_0, align 4, !dbg !1936
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1936
  store i32 3, ptr %8, align 4, !dbg !1936
  br label %bb33, !dbg !1936

bb24:                                             ; preds = %start
  store ptr @alloc_172396a7e6a4caf1be590182cac51572, ptr %_0, align 4, !dbg !1937
  %9 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1937
  store i32 3, ptr %9, align 4, !dbg !1937
  br label %bb33, !dbg !1937

bb23:                                             ; preds = %start
  store ptr @alloc_80a21bb18cfbe63f6862ef423ec1eb36, ptr %_0, align 4, !dbg !1938
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1938
  store i32 4, ptr %10, align 4, !dbg !1938
  br label %bb33, !dbg !1938

bb22:                                             ; preds = %start
  store ptr @alloc_07a1d14791a230a5109bcf389757953c, ptr %_0, align 4, !dbg !1939
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1939
  store i32 4, ptr %11, align 4, !dbg !1939
  br label %bb33, !dbg !1939

bb21:                                             ; preds = %start
  store ptr @alloc_9fae5217b901fa82c8a06bff12e6775f, ptr %_0, align 4, !dbg !1940
  %12 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1940
  store i32 4, ptr %12, align 4, !dbg !1940
  br label %bb33, !dbg !1940

bb20:                                             ; preds = %start
  store ptr @alloc_544a5f255a52c55972be2a457da1e695, ptr %_0, align 4, !dbg !1941
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1941
  store i32 4, ptr %13, align 4, !dbg !1941
  br label %bb33, !dbg !1941

bb19:                                             ; preds = %start
  store ptr @alloc_4832908c0d36609818b44d52da3b4ab1, ptr %_0, align 4, !dbg !1942
  %14 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1942
  store i32 4, ptr %14, align 4, !dbg !1942
  br label %bb33, !dbg !1942

bb18:                                             ; preds = %start
  store ptr @alloc_a2e52520915a0d17d404534001d33e86, ptr %_0, align 4, !dbg !1943
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1943
  store i32 4, ptr %15, align 4, !dbg !1943
  br label %bb33, !dbg !1943

bb17:                                             ; preds = %start
  store ptr @alloc_c940f1872184b67533cde325d4eb7ceb, ptr %_0, align 4, !dbg !1944
  %16 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1944
  store i32 4, ptr %16, align 4, !dbg !1944
  br label %bb33, !dbg !1944

bb16:                                             ; preds = %start
  store ptr @alloc_700eaf7a86f77d80c28b2adbf3cc9803, ptr %_0, align 4, !dbg !1945
  %17 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1945
  store i32 4, ptr %17, align 4, !dbg !1945
  br label %bb33, !dbg !1945

bb15:                                             ; preds = %start
  store ptr @alloc_1e6c27aaa4a4f9cb003486cc93d37053, ptr %_0, align 4, !dbg !1946
  %18 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1946
  store i32 4, ptr %18, align 4, !dbg !1946
  br label %bb33, !dbg !1946

bb14:                                             ; preds = %start
  store ptr @alloc_916b433ff317c0fd3edfdd465d6ba8b9, ptr %_0, align 4, !dbg !1947
  %19 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1947
  store i32 4, ptr %19, align 4, !dbg !1947
  br label %bb33, !dbg !1947

bb13:                                             ; preds = %start
  store ptr @alloc_9cc6a38345689c915b47d594cb019fed, ptr %_0, align 4, !dbg !1948
  %20 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1948
  store i32 4, ptr %20, align 4, !dbg !1948
  br label %bb33, !dbg !1948

bb12:                                             ; preds = %start
  store ptr @alloc_c4577f07e7c9ccf92dd42115bc7131a5, ptr %_0, align 4, !dbg !1949
  %21 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1949
  store i32 4, ptr %21, align 4, !dbg !1949
  br label %bb33, !dbg !1949

bb11:                                             ; preds = %start
  store ptr @alloc_32f6530f1dc9b98127bcb811b0532ba7, ptr %_0, align 4, !dbg !1950
  %22 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1950
  store i32 4, ptr %22, align 4, !dbg !1950
  br label %bb33, !dbg !1950

bb10:                                             ; preds = %start
  store ptr @alloc_36c7b7b3d4a1a29180c38bf79b66c057, ptr %_0, align 4, !dbg !1951
  %23 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1951
  store i32 3, ptr %23, align 4, !dbg !1951
  br label %bb33, !dbg !1951

bb9:                                              ; preds = %start
  store ptr @alloc_7eb7520302da06d239e69560c3e0708c, ptr %_0, align 4, !dbg !1952
  %24 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1952
  store i32 4, ptr %24, align 4, !dbg !1952
  br label %bb33, !dbg !1952

bb8:                                              ; preds = %start
  store ptr @alloc_40234d469372e9f8ef518dd54a19727f, ptr %_0, align 4, !dbg !1953
  %25 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1953
  store i32 4, ptr %25, align 4, !dbg !1953
  br label %bb33, !dbg !1953

bb7:                                              ; preds = %start
  store ptr @alloc_31643e4a78119bdf0ff87de45542db06, ptr %_0, align 4, !dbg !1954
  %26 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1954
  store i32 6, ptr %26, align 4, !dbg !1954
  br label %bb33, !dbg !1954

bb6:                                              ; preds = %start
  store ptr @alloc_42106c4e8e5f57f6c471196129c7ada1, ptr %_0, align 4, !dbg !1955
  %27 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1955
  store i32 4, ptr %27, align 4, !dbg !1955
  br label %bb33, !dbg !1955

bb5:                                              ; preds = %start
  store ptr @alloc_eb9a91982746a8bf460d0a0d2c98fe81, ptr %_0, align 4, !dbg !1956
  %28 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1956
  store i32 5, ptr %28, align 4, !dbg !1956
  br label %bb33, !dbg !1956

bb4:                                              ; preds = %start
  store ptr @alloc_d86c1c0ecbe01a4015494d5a0a7bd4e8, ptr %_0, align 4, !dbg !1957
  %29 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1957
  store i32 4, ptr %29, align 4, !dbg !1957
  br label %bb33, !dbg !1957

bb3:                                              ; preds = %start
  store ptr @alloc_1d9e67eff05b636a4dea07e14807617f, ptr %_0, align 4, !dbg !1958
  %30 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1958
  store i32 3, ptr %30, align 4, !dbg !1958
  br label %bb33, !dbg !1958

bb2:                                              ; preds = %start
  store ptr @alloc_13d53d5a7d472d642bc2ba5eddd00818, ptr %_0, align 4, !dbg !1959
  %31 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1959
  store i32 3, ptr %31, align 4, !dbg !1959
  br label %bb33, !dbg !1959

bb33:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23, %bb24, %bb25, %bb26, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32
  %32 = load ptr, ptr %_0, align 4, !dbg !1960
  %33 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1960
  %34 = load i32, ptr %33, align 4, !dbg !1960
  %35 = insertvalue { ptr, i32 } poison, ptr %32, 0, !dbg !1960
  %36 = insertvalue { ptr, i32 } %35, i32 %34, 1, !dbg !1960
  ret { ptr, i32 } %36, !dbg !1960
}

; wasi::lib_generated::Signal::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Signal7message17h5c961ce5e1cd5f15E(ptr align 1 %self) unnamed_addr #2 !dbg !1961 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !1964, !DIExpression(), !1965)
  %0 = load i8, ptr %self, align 1, !dbg !1966
  switch i8 %0, label %bb1 [
    i8 0, label %bb32
    i8 1, label %bb31
    i8 2, label %bb30
    i8 3, label %bb29
    i8 4, label %bb28
    i8 5, label %bb27
    i8 6, label %bb26
    i8 7, label %bb25
    i8 8, label %bb24
    i8 9, label %bb23
    i8 10, label %bb22
    i8 11, label %bb21
    i8 12, label %bb20
    i8 13, label %bb19
    i8 14, label %bb18
    i8 15, label %bb17
    i8 16, label %bb16
    i8 17, label %bb15
    i8 18, label %bb14
    i8 19, label %bb13
    i8 20, label %bb12
    i8 21, label %bb11
    i8 22, label %bb10
    i8 23, label %bb9
    i8 24, label %bb8
    i8 25, label %bb7
    i8 26, label %bb6
    i8 27, label %bb5
    i8 28, label %bb4
    i8 29, label %bb3
    i8 30, label %bb2
  ], !dbg !1966

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_53d68276a954c11b13aab7829171cada) #53, !dbg !1967
  unreachable, !dbg !1967

bb32:                                             ; preds = %start
  store ptr @alloc_1423cf9b1285dc2276463c9080968c6e, ptr %_0, align 4, !dbg !1968
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1968
  store i32 95, ptr %1, align 4, !dbg !1968
  br label %bb33, !dbg !1968

bb31:                                             ; preds = %start
  store ptr @alloc_d1463c6d690f4195e6ae033efbc5e8aa, ptr %_0, align 4, !dbg !1969
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1969
  store i32 39, ptr %2, align 4, !dbg !1969
  br label %bb33, !dbg !1969

bb30:                                             ; preds = %start
  store ptr @alloc_41d2790e9dad417b78c2043eab3b612b, ptr %_0, align 4, !dbg !1970
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1970
  store i32 59, ptr %3, align 4, !dbg !1970
  br label %bb33, !dbg !1970

bb29:                                             ; preds = %start
  store ptr @alloc_5580ea2c7771dcb161701338a11bf185, ptr %_0, align 4, !dbg !1971
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1971
  store i32 53, ptr %4, align 4, !dbg !1971
  br label %bb33, !dbg !1971

bb28:                                             ; preds = %start
  store ptr @alloc_248a2f19574abf8c31e56d6193a6bc66, ptr %_0, align 4, !dbg !1972
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1972
  store i32 52, ptr %5, align 4, !dbg !1972
  br label %bb33, !dbg !1972

bb27:                                             ; preds = %start
  store ptr @alloc_493a384016f4bf3e4c3889a7faf9e4e3, ptr %_0, align 4, !dbg !1973
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1973
  store i32 54, ptr %6, align 4, !dbg !1973
  br label %bb33, !dbg !1973

bb26:                                             ; preds = %start
  store ptr @alloc_ed92c62a78c492f3c7b01873f82aa31e, ptr %_0, align 4, !dbg !1974
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1974
  store i32 53, ptr %7, align 4, !dbg !1974
  br label %bb33, !dbg !1974

bb25:                                             ; preds = %start
  store ptr @alloc_84a64b8254125961372be19e65c94f49, ptr %_0, align 4, !dbg !1975
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1975
  store i32 82, ptr %8, align 4, !dbg !1975
  br label %bb33, !dbg !1975

bb24:                                             ; preds = %start
  store ptr @alloc_80bf056227e060e13fa753e3e1de892d, ptr %_0, align 4, !dbg !1976
  %9 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1976
  store i32 63, ptr %9, align 4, !dbg !1976
  br label %bb33, !dbg !1976

bb23:                                             ; preds = %start
  store ptr @alloc_2c10eaca4ad1b29d69d589d9e9e296ac, ptr %_0, align 4, !dbg !1977
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1977
  store i32 37, ptr %10, align 4, !dbg !1977
  br label %bb33, !dbg !1977

bb22:                                             ; preds = %start
  store ptr @alloc_356d3fe077f6c83805a2b19a2b21ae2f, ptr %_0, align 4, !dbg !1978
  %11 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1978
  store i32 54, ptr %11, align 4, !dbg !1978
  br label %bb33, !dbg !1978

bb21:                                             ; preds = %start
  store ptr @alloc_9f15daac07f269ee6fd44f89ce0f59bf, ptr %_0, align 4, !dbg !1979
  %12 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1979
  store i32 57, ptr %12, align 4, !dbg !1979
  br label %bb33, !dbg !1979

bb20:                                             ; preds = %start
  store ptr @alloc_689faaf1d42056dc8b2f636f12486583, ptr %_0, align 4, !dbg !1980
  %13 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1980
  store i32 54, ptr %13, align 4, !dbg !1980
  br label %bb33, !dbg !1980

bb19:                                             ; preds = %start
  store ptr @alloc_2200f213787ac008059970ba784239a7, ptr %_0, align 4, !dbg !1981
  %14 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1981
  store i32 56, ptr %14, align 4, !dbg !1981
  br label %bb33, !dbg !1981

bb18:                                             ; preds = %start
  store ptr @alloc_2858c2305fa03fdc3d3c76407e2cc263, ptr %_0, align 4, !dbg !1982
  %15 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1982
  store i32 44, ptr %15, align 4, !dbg !1982
  br label %bb33, !dbg !1982

bb17:                                             ; preds = %start
  store ptr @alloc_c3e8042022d30e3d9443605454f05952, ptr %_0, align 4, !dbg !1983
  %16 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1983
  store i32 51, ptr %16, align 4, !dbg !1983
  br label %bb33, !dbg !1983

bb16:                                             ; preds = %start
  store ptr @alloc_0a31d2078b214ff88d1994cf10051d5a, ptr %_0, align 4, !dbg !1984
  %17 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1984
  store i32 65, ptr %17, align 4, !dbg !1984
  br label %bb33, !dbg !1984

bb15:                                             ; preds = %start
  store ptr @alloc_745dc892bfb3506c1d83e31c0d775108, ptr %_0, align 4, !dbg !1985
  %18 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1985
  store i32 72, ptr %18, align 4, !dbg !1985
  br label %bb33, !dbg !1985

bb14:                                             ; preds = %start
  store ptr @alloc_a59eb384144ffa9b0a65ee678a07f52c, ptr %_0, align 4, !dbg !1986
  %19 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1986
  store i32 40, ptr %19, align 4, !dbg !1986
  br label %bb33, !dbg !1986

bb13:                                             ; preds = %start
  store ptr @alloc_17a4a3403af04ca62d45bd3a94aa5655, ptr %_0, align 4, !dbg !1987
  %20 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1987
  store i32 46, ptr %20, align 4, !dbg !1987
  br label %bb33, !dbg !1987

bb12:                                             ; preds = %start
  store ptr @alloc_58995edd6c7ab0cd8cef2af5cecf15ef, ptr %_0, align 4, !dbg !1988
  %21 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1988
  store i32 60, ptr %21, align 4, !dbg !1988
  br label %bb33, !dbg !1988

bb11:                                             ; preds = %start
  store ptr @alloc_460288a5f6a2eec462f5597a92f2d641, ptr %_0, align 4, !dbg !1989
  %22 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1989
  store i32 61, ptr %22, align 4, !dbg !1989
  br label %bb33, !dbg !1989

bb10:                                             ; preds = %start
  store ptr @alloc_45d2b94afd2e05ef565c22d943786616, ptr %_0, align 4, !dbg !1990
  %23 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1990
  store i32 62, ptr %23, align 4, !dbg !1990
  br label %bb33, !dbg !1990

bb9:                                              ; preds = %start
  store ptr @alloc_c2e111ecc720ba84b14250e38b86b16d, ptr %_0, align 4, !dbg !1991
  %24 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1991
  store i32 56, ptr %24, align 4, !dbg !1991
  br label %bb33, !dbg !1991

bb8:                                              ; preds = %start
  store ptr @alloc_74a3ad70801e06f3bfe7505d1789a8f4, ptr %_0, align 4, !dbg !1992
  %25 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1992
  store i32 57, ptr %25, align 4, !dbg !1992
  br label %bb33, !dbg !1992

bb7:                                              ; preds = %start
  store ptr @alloc_c198d2f1aabfc2bab249933e1ee4eb1c, ptr %_0, align 4, !dbg !1993
  %26 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1993
  store i32 54, ptr %26, align 4, !dbg !1993
  br label %bb33, !dbg !1993

bb6:                                              ; preds = %start
  store ptr @alloc_20bb6bf3f1ace7525c9405f2282207b6, ptr %_0, align 4, !dbg !1994
  %27 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1994
  store i32 56, ptr %27, align 4, !dbg !1994
  br label %bb33, !dbg !1994

bb5:                                              ; preds = %start
  store ptr @alloc_150b5f9befb6857dc7daa49e9e6fbc95, ptr %_0, align 4, !dbg !1995
  %28 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1995
  store i32 32, ptr %28, align 4, !dbg !1995
  br label %bb33, !dbg !1995

bb4:                                              ; preds = %start
  store ptr @alloc_6ab3986896163e2ad59f1f57a6e1519f, ptr %_0, align 4, !dbg !1996
  %29 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1996
  store i32 45, ptr %29, align 4, !dbg !1996
  br label %bb33, !dbg !1996

bb3:                                              ; preds = %start
  store ptr @alloc_a4c30caabbc8ee013e8bfac02e84c080, ptr %_0, align 4, !dbg !1997
  %30 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1997
  store i32 46, ptr %30, align 4, !dbg !1997
  br label %bb33, !dbg !1997

bb2:                                              ; preds = %start
  store ptr @alloc_97b2e00b5705f2c9646cc82f6fb99c61, ptr %_0, align 4, !dbg !1998
  %31 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1998
  store i32 48, ptr %31, align 4, !dbg !1998
  br label %bb33, !dbg !1998

bb33:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23, %bb24, %bb25, %bb26, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32
  %32 = load ptr, ptr %_0, align 4, !dbg !1999
  %33 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !1999
  %34 = load i32, ptr %33, align 4, !dbg !1999
  %35 = insertvalue { ptr, i32 } poison, ptr %32, 0, !dbg !1999
  %36 = insertvalue { ptr, i32 } %35, i32 %34, 1, !dbg !1999
  ret { ptr, i32 } %36, !dbg !1999
}

; wasi::lib_generated::Whence::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated6Whence3raw17ha742762abb60085dE(ptr align 1 %self) unnamed_addr #2 !dbg !2000 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2009, !DIExpression(), !2010)
  %_0 = load i8, ptr %self, align 1, !dbg !2011
  ret i8 %_0, !dbg !2012
}

; wasi::lib_generated::Whence::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Whence4name17he958b0be86dcb2e2E(ptr align 1 %self) unnamed_addr #2 !dbg !2013 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2018, !DIExpression(), !2019)
  %0 = load i8, ptr %self, align 1, !dbg !2020
  switch i8 %0, label %bb1 [
    i8 0, label %bb4
    i8 1, label %bb3
    i8 2, label %bb2
  ], !dbg !2020

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_c07eba15eca988477b5fb63c67d540a8) #53, !dbg !2021
  unreachable, !dbg !2021

bb4:                                              ; preds = %start
  store ptr @alloc_5db363c82b07540837cd27a17f0c663b, ptr %_0, align 4, !dbg !2022
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2022
  store i32 3, ptr %1, align 4, !dbg !2022
  br label %bb5, !dbg !2022

bb3:                                              ; preds = %start
  store ptr @alloc_ff6ccd04681148d223fbb1fb8e2efc21, ptr %_0, align 4, !dbg !2023
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2023
  store i32 3, ptr %2, align 4, !dbg !2023
  br label %bb5, !dbg !2023

bb2:                                              ; preds = %start
  store ptr @alloc_29db745e090ad0183dfdc88ed77b1433, ptr %_0, align 4, !dbg !2024
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2024
  store i32 3, ptr %3, align 4, !dbg !2024
  br label %bb5, !dbg !2024

bb5:                                              ; preds = %bb2, %bb3, %bb4
  %4 = load ptr, ptr %_0, align 4, !dbg !2025
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2025
  %6 = load i32, ptr %5, align 4, !dbg !2025
  %7 = insertvalue { ptr, i32 } poison, ptr %4, 0, !dbg !2025
  %8 = insertvalue { ptr, i32 } %7, i32 %6, 1, !dbg !2025
  ret { ptr, i32 } %8, !dbg !2025
}

; wasi::lib_generated::Whence::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated6Whence7message17ha9e612d937001f4fE(ptr align 1 %self) unnamed_addr #2 !dbg !2026 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2029, !DIExpression(), !2030)
  %0 = load i8, ptr %self, align 1, !dbg !2031
  switch i8 %0, label %bb1 [
    i8 0, label %bb4
    i8 1, label %bb3
    i8 2, label %bb2
  ], !dbg !2031

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_c7c92b698ef4610640186963f648efe9) #53, !dbg !2032
  unreachable, !dbg !2032

bb4:                                              ; preds = %start
  store ptr @alloc_24a3caf022c395e6ae7e098e8f399198, ptr %_0, align 4, !dbg !2033
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2033
  store i32 31, ptr %1, align 4, !dbg !2033
  br label %bb5, !dbg !2033

bb3:                                              ; preds = %start
  store ptr @alloc_2c2e9693ef801993a434a363e8a8a714, ptr %_0, align 4, !dbg !2034
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2034
  store i32 34, ptr %2, align 4, !dbg !2034
  br label %bb5, !dbg !2034

bb2:                                              ; preds = %start
  store ptr @alloc_10ffe8fd38749369565ce0219086a5bd, ptr %_0, align 4, !dbg !2035
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2035
  store i32 29, ptr %3, align 4, !dbg !2035
  br label %bb5, !dbg !2035

bb5:                                              ; preds = %bb2, %bb3, %bb4
  %4 = load ptr, ptr %_0, align 4, !dbg !2036
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2036
  %6 = load i32, ptr %5, align 4, !dbg !2036
  %7 = insertvalue { ptr, i32 } poison, ptr %4, 0, !dbg !2036
  %8 = insertvalue { ptr, i32 } %7, i32 %6, 1, !dbg !2036
  ret { ptr, i32 } %8, !dbg !2036
}

; wasi::lib_generated::Clockid::raw
; Function Attrs: nounwind
define dso_local i32 @_ZN4wasi13lib_generated7Clockid3raw17h3b57b00042d58b44E(ptr align 4 %self) unnamed_addr #2 !dbg !2037 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2043, !DIExpression(), !2044)
  %_0 = load i32, ptr %self, align 4, !dbg !2045
  ret i32 %_0, !dbg !2046
}

; wasi::lib_generated::Clockid::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated7Clockid4name17h908f75bcb5817523E(ptr align 4 %self) unnamed_addr #2 !dbg !2047 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2052, !DIExpression(), !2053)
  %0 = load i32, ptr %self, align 4, !dbg !2054
  switch i32 %0, label %bb1 [
    i32 0, label %bb5
    i32 1, label %bb4
    i32 2, label %bb3
    i32 3, label %bb2
  ], !dbg !2054

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_8bbd04dca6dc62dbeef9758be427fde9) #53, !dbg !2055
  unreachable, !dbg !2055

bb5:                                              ; preds = %start
  store ptr @alloc_a902d1a7a762d9fe1e7935df74c14685, ptr %_0, align 4, !dbg !2056
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2056
  store i32 8, ptr %1, align 4, !dbg !2056
  br label %bb6, !dbg !2056

bb4:                                              ; preds = %start
  store ptr @alloc_b430f478269159bc9daab7abd375695f, ptr %_0, align 4, !dbg !2057
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2057
  store i32 9, ptr %2, align 4, !dbg !2057
  br label %bb6, !dbg !2057

bb3:                                              ; preds = %start
  store ptr @alloc_2e6987dab4e18cfd64262ee6f5a1f05b, ptr %_0, align 4, !dbg !2058
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2058
  store i32 18, ptr %3, align 4, !dbg !2058
  br label %bb6, !dbg !2058

bb2:                                              ; preds = %start
  store ptr @alloc_34477b3239657b53925e478a823eef9e, ptr %_0, align 4, !dbg !2059
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2059
  store i32 17, ptr %4, align 4, !dbg !2059
  br label %bb6, !dbg !2059

bb6:                                              ; preds = %bb2, %bb3, %bb4, %bb5
  %5 = load ptr, ptr %_0, align 4, !dbg !2060
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2060
  %7 = load i32, ptr %6, align 4, !dbg !2060
  %8 = insertvalue { ptr, i32 } poison, ptr %5, 0, !dbg !2060
  %9 = insertvalue { ptr, i32 } %8, i32 %7, 1, !dbg !2060
  ret { ptr, i32 } %9, !dbg !2060
}

; wasi::lib_generated::Clockid::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated7Clockid7message17hf15037538852fe88E(ptr align 4 %self) unnamed_addr #2 !dbg !2061 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2064, !DIExpression(), !2065)
  %0 = load i32, ptr %self, align 4, !dbg !2066
  switch i32 %0, label %bb1 [
    i32 0, label %bb5
    i32 1, label %bb4
    i32 2, label %bb3
    i32 3, label %bb2
  ], !dbg !2066

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_e3f936e04555125f465e22d7f3bb2614) #53, !dbg !2067
  unreachable, !dbg !2067

bb5:                                              ; preds = %start
  store ptr @alloc_1aea4d167b9a42393df22a71e3a8f11b, ptr %_0, align 4, !dbg !2068
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2068
  store i32 85, ptr %1, align 4, !dbg !2068
  br label %bb6, !dbg !2068

bb4:                                              ; preds = %start
  store ptr @alloc_5bfa82a09c61ee9eec58df8e794b5e7a, ptr %_0, align 4, !dbg !2069
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2069
  store i32 257, ptr %2, align 4, !dbg !2069
  br label %bb6, !dbg !2069

bb3:                                              ; preds = %start
  store ptr @alloc_2d54b354caf70a3f2c540cc2dfd4a0b8, ptr %_0, align 4, !dbg !2070
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2070
  store i32 55, ptr %3, align 4, !dbg !2070
  br label %bb6, !dbg !2070

bb2:                                              ; preds = %start
  store ptr @alloc_e7d7dc92d73b7cdd8f44c10b5d3c455b, ptr %_0, align 4, !dbg !2071
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2071
  store i32 54, ptr %4, align 4, !dbg !2071
  br label %bb6, !dbg !2071

bb6:                                              ; preds = %bb2, %bb3, %bb4, %bb5
  %5 = load ptr, ptr %_0, align 4, !dbg !2072
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2072
  %7 = load i32, ptr %6, align 4, !dbg !2072
  %8 = insertvalue { ptr, i32 } poison, ptr %5, 0, !dbg !2072
  %9 = insertvalue { ptr, i32 } %8, i32 %7, 1, !dbg !2072
  ret { ptr, i32 } %9, !dbg !2072
}

; wasi::lib_generated::fd_read
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated7fd_read17hb0dacab1222e8d36E(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 4 %iovs.0, i32 %iovs.1) unnamed_addr #2 !dbg !2073 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %iovs.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2086, !DIExpression(), !2092)
  store ptr %iovs.0, ptr %iovs.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %iovs.dbg.spill, i32 4
  store i32 %iovs.1, ptr %0, align 4
    #dbg_declare(ptr %iovs.dbg.spill, !2087, !DIExpression(), !2093)
    #dbg_declare(ptr %rp0, !2088, !DIExpression(), !2094)
  store i32 undef, ptr %rp0, align 4, !dbg !2095
  store ptr %iovs.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %iovs.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2096, !DIExpression(), !2107)
  %_6 = ptrtoint ptr %iovs.0 to i32, !dbg !2109
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2110)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !2112
; call wasi::lib_generated::wasi_snapshot_preview1::fd_read
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_read17hb9636e7cb0eba53eE(i32 %fd, i32 %_6, i32 %iovs.1, i32 %_10) #52, !dbg !2113
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2113
    #dbg_declare(ptr %ret.dbg.spill, !2090, !DIExpression(), !2114)
  %2 = icmp eq i32 %ret, 0, !dbg !2115
  br i1 %2, label %bb6, label %bb5, !dbg !2115

bb6:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2116)
  %_15 = ptrtoint ptr %rp0 to i32, !dbg !2118
  %_14 = inttoptr i32 %_15 to ptr, !dbg !2118
; call core::ptr::read
  %_13 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_14, ptr align 4 @alloc_70e39a2b9d2b22656a4b6352fc00abaa) #52, !dbg !2119
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2120
  store i32 %_13, ptr %3, align 4, !dbg !2120
  store i16 0, ptr %_0, align 4, !dbg !2120
  br label %bb9, !dbg !2121

bb5:                                              ; preds = %start
  %_19 = trunc i32 %ret to i16, !dbg !2122
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2123
  store i16 %_19, ptr %4, align 2, !dbg !2123
  store i16 1, ptr %_0, align 4, !dbg !2123
  br label %bb9, !dbg !2124

bb9:                                              ; preds = %bb5, %bb6
  ret void, !dbg !2125
}

; wasi::lib_generated::fd_seek
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated7fd_seek17h9e7fb5fc4b342c97E(ptr sret([16 x i8]) align 8 %_0, i32 %fd, i64 %offset, i8 %whence) unnamed_addr #2 !dbg !2126 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %whence.dbg.spill = alloca [1 x i8], align 1
  %offset.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [8 x i8], align 8
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2131, !DIExpression(), !2138)
  store i64 %offset, ptr %offset.dbg.spill, align 8
    #dbg_declare(ptr %offset.dbg.spill, !2132, !DIExpression(), !2139)
  store i8 %whence, ptr %whence.dbg.spill, align 1
    #dbg_declare(ptr %whence.dbg.spill, !2133, !DIExpression(), !2140)
    #dbg_declare(ptr %rp0, !2134, !DIExpression(), !2141)
  store i64 undef, ptr %rp0, align 8, !dbg !2142
  %_7 = zext i8 %whence to i32, !dbg !2143
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1077, !DIExpression(), !2144)
  %_9 = ptrtoint ptr %rp0 to i32, !dbg !2146
; call wasi::lib_generated::wasi_snapshot_preview1::fd_seek
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_seek17hc5a5c23fbf7b77adE(i32 %fd, i64 %offset, i32 %_7, i32 %_9) #52, !dbg !2147
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2147
    #dbg_declare(ptr %ret.dbg.spill, !2136, !DIExpression(), !2148)
  %0 = icmp eq i32 %ret, 0, !dbg !2149
  br i1 %0, label %bb5, label %bb4, !dbg !2149

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1077, !DIExpression(), !2150)
  %_14 = ptrtoint ptr %rp0 to i32, !dbg !2152
  %_13 = inttoptr i32 %_14 to ptr, !dbg !2152
; call core::ptr::read
  %_12 = call i64 @_ZN4core3ptr4read17h487dc6145fad69b1E(ptr %_13, ptr align 4 @alloc_ea3c79e2bbf0f7b9771412f9a1555d86) #52, !dbg !2153
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !2154
  store i64 %_12, ptr %1, align 8, !dbg !2154
  store i16 0, ptr %_0, align 8, !dbg !2154
  br label %bb8, !dbg !2155

bb4:                                              ; preds = %start
  %_18 = trunc i32 %ret to i16, !dbg !2156
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2157
  store i16 %_18, ptr %2, align 2, !dbg !2157
  store i16 1, ptr %_0, align 8, !dbg !2157
  br label %bb8, !dbg !2158

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !2159
}

; wasi::lib_generated::fd_sync
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated7fd_sync17h0245601cea21c804E(i32 %fd) unnamed_addr #2 !dbg !2160 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2162, !DIExpression(), !2165)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_sync
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_sync17h00da7a4290b09810E(i32 %fd) #52, !dbg !2166
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2166
    #dbg_declare(ptr %ret.dbg.spill, !2163, !DIExpression(), !2167)
  %0 = icmp eq i32 %ret, 0, !dbg !2168
  br i1 %0, label %bb3, label %bb2, !dbg !2168

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !2169
  br label %bb4, !dbg !2170

bb2:                                              ; preds = %start
  %_5 = trunc i32 %ret to i16, !dbg !2171
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2172
  store i16 %_5, ptr %1, align 2, !dbg !2172
  store i16 1, ptr %_0, align 2, !dbg !2172
  br label %bb4, !dbg !2173

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !2174
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2174
  %4 = load i16, ptr %3, align 2, !dbg !2174
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !2174
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !2174
  ret { i16, i16 } %6, !dbg !2174
}

; wasi::lib_generated::fd_tell
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated7fd_tell17h7dc9dda5a5637ea0E(ptr sret([16 x i8]) align 8 %_0, i32 %fd) unnamed_addr #2 !dbg !2175 {
start:
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [8 x i8], align 8
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2179, !DIExpression(), !2184)
    #dbg_declare(ptr %rp0, !2180, !DIExpression(), !2185)
  store i64 undef, ptr %rp0, align 8, !dbg !2186
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !1077, !DIExpression(), !2187)
  %_5 = ptrtoint ptr %rp0 to i32, !dbg !2189
; call wasi::lib_generated::wasi_snapshot_preview1::fd_tell
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_tell17hb423faf6c354b520E(i32 %fd, i32 %_5) #52, !dbg !2190
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2190
    #dbg_declare(ptr %ret.dbg.spill, !2182, !DIExpression(), !2191)
  %0 = icmp eq i32 %ret, 0, !dbg !2192
  br i1 %0, label %bb5, label %bb4, !dbg !2192

bb5:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !1077, !DIExpression(), !2193)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !2195
  %_9 = inttoptr i32 %_10 to ptr, !dbg !2195
; call core::ptr::read
  %_8 = call i64 @_ZN4core3ptr4read17h487dc6145fad69b1E(ptr %_9, ptr align 4 @alloc_1364e2512203bc4925076d374c123589) #52, !dbg !2196
  %1 = getelementptr inbounds i8, ptr %_0, i32 8, !dbg !2197
  store i64 %_8, ptr %1, align 8, !dbg !2197
  store i16 0, ptr %_0, align 8, !dbg !2197
  br label %bb8, !dbg !2198

bb4:                                              ; preds = %start
  %_14 = trunc i32 %ret to i16, !dbg !2199
  %2 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2200
  store i16 %_14, ptr %2, align 2, !dbg !2200
  store i16 1, ptr %_0, align 8, !dbg !2200
  br label %bb8, !dbg !2201

bb8:                                              ; preds = %bb4, %bb5
  ret void, !dbg !2202
}

; wasi::lib_generated::Filetype::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated8Filetype3raw17h3070f49fd500d0dfE(ptr align 1 %self) unnamed_addr #2 !dbg !2203 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2209, !DIExpression(), !2210)
  %_0 = load i8, ptr %self, align 1, !dbg !2211
  ret i8 %_0, !dbg !2212
}

; wasi::lib_generated::Filetype::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated8Filetype4name17h42a52d431e4e0e3fE(ptr align 1 %self) unnamed_addr #2 !dbg !2213 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2218, !DIExpression(), !2219)
  %0 = load i8, ptr %self, align 1, !dbg !2220
  switch i8 %0, label %bb1 [
    i8 0, label %bb9
    i8 1, label %bb8
    i8 2, label %bb7
    i8 3, label %bb6
    i8 4, label %bb5
    i8 5, label %bb4
    i8 6, label %bb3
    i8 7, label %bb2
  ], !dbg !2220

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_3b304d6af034541cbe6c1768e0d7e144) #53, !dbg !2221
  unreachable, !dbg !2221

bb9:                                              ; preds = %start
  store ptr @alloc_3e9eb586b0e75631ded4bff06697b5ef, ptr %_0, align 4, !dbg !2222
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2222
  store i32 7, ptr %1, align 4, !dbg !2222
  br label %bb10, !dbg !2222

bb8:                                              ; preds = %start
  store ptr @alloc_6ad3f50235942c9b42c0e7e5cb6f0f1c, ptr %_0, align 4, !dbg !2223
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2223
  store i32 12, ptr %2, align 4, !dbg !2223
  br label %bb10, !dbg !2223

bb7:                                              ; preds = %start
  store ptr @alloc_2478ed23cecb242b65d611620b0117f3, ptr %_0, align 4, !dbg !2224
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2224
  store i32 16, ptr %3, align 4, !dbg !2224
  br label %bb10, !dbg !2224

bb6:                                              ; preds = %start
  store ptr @alloc_9a62b518932bfe53ea52a4b3adc16526, ptr %_0, align 4, !dbg !2225
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2225
  store i32 9, ptr %4, align 4, !dbg !2225
  br label %bb10, !dbg !2225

bb5:                                              ; preds = %start
  store ptr @alloc_abfb6e9fa12f0cc71c3152a03fa9d825, ptr %_0, align 4, !dbg !2226
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2226
  store i32 12, ptr %5, align 4, !dbg !2226
  br label %bb10, !dbg !2226

bb4:                                              ; preds = %start
  store ptr @alloc_3ebef2482db8a977a988cb55208895b9, ptr %_0, align 4, !dbg !2227
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2227
  store i32 12, ptr %6, align 4, !dbg !2227
  br label %bb10, !dbg !2227

bb3:                                              ; preds = %start
  store ptr @alloc_2de38242575aec0efc9f447ec8601b4f, ptr %_0, align 4, !dbg !2228
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2228
  store i32 13, ptr %7, align 4, !dbg !2228
  br label %bb10, !dbg !2228

bb2:                                              ; preds = %start
  store ptr @alloc_34ce0fe5a4fa95f9a500f0adc894366a, ptr %_0, align 4, !dbg !2229
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2229
  store i32 13, ptr %8, align 4, !dbg !2229
  br label %bb10, !dbg !2229

bb10:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9
  %9 = load ptr, ptr %_0, align 4, !dbg !2230
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2230
  %11 = load i32, ptr %10, align 4, !dbg !2230
  %12 = insertvalue { ptr, i32 } poison, ptr %9, 0, !dbg !2230
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1, !dbg !2230
  ret { ptr, i32 } %13, !dbg !2230
}

; wasi::lib_generated::Filetype::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated8Filetype7message17hc28156d203146fb1E(ptr align 1 %self) unnamed_addr #2 !dbg !2231 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2234, !DIExpression(), !2235)
  %0 = load i8, ptr %self, align 1, !dbg !2236
  switch i8 %0, label %bb1 [
    i8 0, label %bb9
    i8 1, label %bb8
    i8 2, label %bb7
    i8 3, label %bb6
    i8 4, label %bb5
    i8 5, label %bb4
    i8 6, label %bb3
    i8 7, label %bb2
  ], !dbg !2236

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_6c0a2912748dcff49d64307d3d52e0a7) #53, !dbg !2237
  unreachable, !dbg !2237

bb9:                                              ; preds = %start
  store ptr @alloc_2881e96a1d6c2134e67cfef9638c5e77, ptr %_0, align 4, !dbg !2238
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2238
  store i32 105, ptr %1, align 4, !dbg !2238
  br label %bb10, !dbg !2238

bb8:                                              ; preds = %start
  store ptr @alloc_afa6319c5229c6b9c6d42c58cbe4ad8e, ptr %_0, align 4, !dbg !2239
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2239
  store i32 59, ptr %2, align 4, !dbg !2239
  br label %bb10, !dbg !2239

bb7:                                              ; preds = %start
  store ptr @alloc_e6bbd4acfa578c551423adc2eadde5ff, ptr %_0, align 4, !dbg !2240
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2240
  store i32 63, ptr %3, align 4, !dbg !2240
  br label %bb10, !dbg !2240

bb6:                                              ; preds = %start
  store ptr @alloc_2eabc60a4165c9f7a9418151f63bad00, ptr %_0, align 4, !dbg !2241
  %4 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2241
  store i32 56, ptr %4, align 4, !dbg !2241
  br label %bb10, !dbg !2241

bb5:                                              ; preds = %start
  store ptr @alloc_85306140420f21856a39130a15774ba4, ptr %_0, align 4, !dbg !2242
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2242
  store i32 59, ptr %5, align 4, !dbg !2242
  br label %bb10, !dbg !2242

bb4:                                              ; preds = %start
  store ptr @alloc_5e9de2c7f264779b97c469a088a5aff8, ptr %_0, align 4, !dbg !2243
  %6 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2243
  store i32 56, ptr %6, align 4, !dbg !2243
  br label %bb10, !dbg !2243

bb3:                                              ; preds = %start
  store ptr @alloc_8a52f02783a958636b88c874240e7bba, ptr %_0, align 4, !dbg !2244
  %7 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2244
  store i32 59, ptr %7, align 4, !dbg !2244
  br label %bb10, !dbg !2244

bb2:                                              ; preds = %start
  store ptr @alloc_10be943b3d59ff9aac6f8e41e2c46891, ptr %_0, align 4, !dbg !2245
  %8 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2245
  store i32 41, ptr %8, align 4, !dbg !2245
  br label %bb10, !dbg !2245

bb10:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9
  %9 = load ptr, ptr %_0, align 4, !dbg !2246
  %10 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2246
  %11 = load i32, ptr %10, align 4, !dbg !2246
  %12 = insertvalue { ptr, i32 } poison, ptr %9, 0, !dbg !2246
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1, !dbg !2246
  ret { ptr, i32 } %13, !dbg !2246
}

; wasi::lib_generated::args_get
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated8args_get17hfd25f001ff1fe64aE(ptr %argv, ptr %argv_buf) unnamed_addr #2 !dbg !2247 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %argv_buf.dbg.spill = alloca [4 x i8], align 4
  %argv.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store ptr %argv, ptr %argv.dbg.spill, align 4
    #dbg_declare(ptr %argv.dbg.spill, !2249, !DIExpression(), !2253)
  store ptr %argv_buf, ptr %argv_buf.dbg.spill, align 4
    #dbg_declare(ptr %argv_buf.dbg.spill, !2250, !DIExpression(), !2254)
  %_4 = ptrtoint ptr %argv to i32, !dbg !2255
  %_5 = ptrtoint ptr %argv_buf to i32, !dbg !2256
; call wasi::lib_generated::wasi_snapshot_preview1::args_get
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18args_get17haf4396b9fc3f17b1E(i32 %_4, i32 %_5) #52, !dbg !2257
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2257
    #dbg_declare(ptr %ret.dbg.spill, !2251, !DIExpression(), !2258)
  %0 = icmp eq i32 %ret, 0, !dbg !2259
  br i1 %0, label %bb3, label %bb2, !dbg !2259

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !2260
  br label %bb4, !dbg !2261

bb2:                                              ; preds = %start
  %_7 = trunc i32 %ret to i16, !dbg !2262
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2263
  store i16 %_7, ptr %1, align 2, !dbg !2263
  store i16 1, ptr %_0, align 2, !dbg !2263
  br label %bb4, !dbg !2264

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !2265
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2265
  %4 = load i16, ptr %3, align 2, !dbg !2265
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !2265
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !2265
  ret { i16, i16 } %6, !dbg !2265
}

; wasi::lib_generated::fd_close
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated8fd_close17hc2388cad40911762E(i32 %fd) unnamed_addr #2 !dbg !2266 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2268, !DIExpression(), !2271)
; call wasi::lib_generated::wasi_snapshot_preview1::fd_close
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_close17he82b66267a7c17f3E(i32 %fd) #52, !dbg !2272
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2272
    #dbg_declare(ptr %ret.dbg.spill, !2269, !DIExpression(), !2273)
  %0 = icmp eq i32 %ret, 0, !dbg !2274
  br i1 %0, label %bb3, label %bb2, !dbg !2274

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !2275
  br label %bb4, !dbg !2276

bb2:                                              ; preds = %start
  %_5 = trunc i32 %ret to i16, !dbg !2277
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2278
  store i16 %_5, ptr %1, align 2, !dbg !2278
  store i16 1, ptr %_0, align 2, !dbg !2278
  br label %bb4, !dbg !2279

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !2280
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2280
  %4 = load i16, ptr %3, align 2, !dbg !2280
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !2280
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !2280
  ret { i16, i16 } %6, !dbg !2280
}

; wasi::lib_generated::fd_pread
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated8fd_pread17hc381ec5dadb084ceE(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 4 %iovs.0, i32 %iovs.1, i64 %offset) unnamed_addr #2 !dbg !2281 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %offset.dbg.spill = alloca [8 x i8], align 8
  %iovs.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2285, !DIExpression(), !2292)
  store ptr %iovs.0, ptr %iovs.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %iovs.dbg.spill, i32 4
  store i32 %iovs.1, ptr %0, align 4
    #dbg_declare(ptr %iovs.dbg.spill, !2286, !DIExpression(), !2293)
  store i64 %offset, ptr %offset.dbg.spill, align 8
    #dbg_declare(ptr %offset.dbg.spill, !2287, !DIExpression(), !2294)
    #dbg_declare(ptr %rp0, !2288, !DIExpression(), !2295)
  store i32 undef, ptr %rp0, align 4, !dbg !2296
  store ptr %iovs.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %iovs.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2096, !DIExpression(), !2297)
  %_7 = ptrtoint ptr %iovs.0 to i32, !dbg !2299
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2300)
  %_12 = ptrtoint ptr %rp0 to i32, !dbg !2302
; call wasi::lib_generated::wasi_snapshot_preview1::fd_pread
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_pread17hcbdc8a7366acee58E(i32 %fd, i32 %_7, i32 %iovs.1, i64 %offset, i32 %_12) #52, !dbg !2303
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2303
    #dbg_declare(ptr %ret.dbg.spill, !2290, !DIExpression(), !2304)
  %2 = icmp eq i32 %ret, 0, !dbg !2305
  br i1 %2, label %bb6, label %bb5, !dbg !2305

bb6:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2306)
  %_17 = ptrtoint ptr %rp0 to i32, !dbg !2308
  %_16 = inttoptr i32 %_17 to ptr, !dbg !2308
; call core::ptr::read
  %_15 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_16, ptr align 4 @alloc_5c468b563f3b3351361a94ed4c508401) #52, !dbg !2309
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2310
  store i32 %_15, ptr %3, align 4, !dbg !2310
  store i16 0, ptr %_0, align 4, !dbg !2310
  br label %bb9, !dbg !2311

bb5:                                              ; preds = %start
  %_21 = trunc i32 %ret to i16, !dbg !2312
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2313
  store i16 %_21, ptr %4, align 2, !dbg !2313
  store i16 1, ptr %_0, align 4, !dbg !2313
  br label %bb9, !dbg !2314

bb9:                                              ; preds = %bb5, %bb6
  ret void, !dbg !2315
}

; wasi::lib_generated::fd_write
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated8fd_write17h4038c76fd62d03c7E(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 4 %iovs.0, i32 %iovs.1) unnamed_addr #2 !dbg !2316 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %iovs.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2329, !DIExpression(), !2335)
  store ptr %iovs.0, ptr %iovs.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %iovs.dbg.spill, i32 4
  store i32 %iovs.1, ptr %0, align 4
    #dbg_declare(ptr %iovs.dbg.spill, !2330, !DIExpression(), !2336)
    #dbg_declare(ptr %rp0, !2331, !DIExpression(), !2337)
  store i32 undef, ptr %rp0, align 4, !dbg !2338
  store ptr %iovs.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %iovs.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2339, !DIExpression(), !2347)
  %_6 = ptrtoint ptr %iovs.0 to i32, !dbg !2349
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2350)
  %_10 = ptrtoint ptr %rp0 to i32, !dbg !2352
; call wasi::lib_generated::wasi_snapshot_preview1::fd_write
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17hdb524b0877e7bdf7E(i32 %fd, i32 %_6, i32 %iovs.1, i32 %_10) #52, !dbg !2353
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2353
    #dbg_declare(ptr %ret.dbg.spill, !2333, !DIExpression(), !2354)
  %2 = icmp eq i32 %ret, 0, !dbg !2355
  br i1 %2, label %bb6, label %bb5, !dbg !2355

bb6:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2356)
  %_15 = ptrtoint ptr %rp0 to i32, !dbg !2358
  %_14 = inttoptr i32 %_15 to ptr, !dbg !2358
; call core::ptr::read
  %_13 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_14, ptr align 4 @alloc_56ddcb5ee63e0bd3d6bfaf3d90916595) #52, !dbg !2359
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2360
  store i32 %_13, ptr %3, align 4, !dbg !2360
  store i16 0, ptr %_0, align 4, !dbg !2360
  br label %bb9, !dbg !2361

bb5:                                              ; preds = %start
  %_19 = trunc i32 %ret to i16, !dbg !2362
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2363
  store i16 %_19, ptr %4, align 2, !dbg !2363
  store i16 1, ptr %_0, align 4, !dbg !2363
  br label %bb9, !dbg !2364

bb9:                                              ; preds = %bb5, %bb6
  ret void, !dbg !2365
}

; wasi::lib_generated::Eventtype::raw
; Function Attrs: nounwind
define dso_local i8 @_ZN4wasi13lib_generated9Eventtype3raw17hf39dfad6b7758084E(ptr align 1 %self) unnamed_addr #2 !dbg !2366 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2372, !DIExpression(), !2373)
  %_0 = load i8, ptr %self, align 1, !dbg !2374
  ret i8 %_0, !dbg !2375
}

; wasi::lib_generated::Eventtype::name
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated9Eventtype4name17h6a88a0e670107d50E(ptr align 1 %self) unnamed_addr #2 !dbg !2376 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2381, !DIExpression(), !2382)
  %0 = load i8, ptr %self, align 1, !dbg !2383
  switch i8 %0, label %bb1 [
    i8 0, label %bb4
    i8 1, label %bb3
    i8 2, label %bb2
  ], !dbg !2383

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_464028a4d23535f5cd5be337a01b3d9f) #53, !dbg !2384
  unreachable, !dbg !2384

bb4:                                              ; preds = %start
  store ptr @alloc_86d080ba8845c5fb85fac7d69d229032, ptr %_0, align 4, !dbg !2385
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2385
  store i32 5, ptr %1, align 4, !dbg !2385
  br label %bb5, !dbg !2385

bb3:                                              ; preds = %start
  store ptr @alloc_0bc353ee913f8c550529f784851f4aad, ptr %_0, align 4, !dbg !2386
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2386
  store i32 7, ptr %2, align 4, !dbg !2386
  br label %bb5, !dbg !2386

bb2:                                              ; preds = %start
  store ptr @alloc_66d727ea1514b68e4733f24afeaa8c0c, ptr %_0, align 4, !dbg !2387
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2387
  store i32 8, ptr %3, align 4, !dbg !2387
  br label %bb5, !dbg !2387

bb5:                                              ; preds = %bb2, %bb3, %bb4
  %4 = load ptr, ptr %_0, align 4, !dbg !2388
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2388
  %6 = load i32, ptr %5, align 4, !dbg !2388
  %7 = insertvalue { ptr, i32 } poison, ptr %4, 0, !dbg !2388
  %8 = insertvalue { ptr, i32 } %7, i32 %6, 1, !dbg !2388
  ret { ptr, i32 } %8, !dbg !2388
}

; wasi::lib_generated::Eventtype::message
; Function Attrs: nounwind
define dso_local { ptr, i32 } @_ZN4wasi13lib_generated9Eventtype7message17h94566f151fe315c9E(ptr align 1 %self) unnamed_addr #2 !dbg !2389 {
start:
  %self.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2392, !DIExpression(), !2393)
  %0 = load i8, ptr %self, align 1, !dbg !2394
  switch i8 %0, label %bb1 [
    i8 0, label %bb4
    i8 1, label %bb3
    i8 2, label %bb2
  ], !dbg !2394

bb1:                                              ; preds = %start
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E(ptr align 4 @alloc_e3313cc9a99b89770b9f98d6811105bb) #53, !dbg !2395
  unreachable, !dbg !2395

bb4:                                              ; preds = %start
  store ptr @alloc_de72e5c289b10c05a483a1229e0c86ca, ptr %_0, align 4, !dbg !2396
  %1 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2396
  store i32 101, ptr %1, align 4, !dbg !2396
  br label %bb5, !dbg !2396

bb3:                                              ; preds = %start
  store ptr @alloc_3273b5a611ac24128db1660de333aacf, ptr %_0, align 4, !dbg !2397
  %2 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2397
  store i32 138, ptr %2, align 4, !dbg !2397
  br label %bb5, !dbg !2397

bb2:                                              ; preds = %start
  store ptr @alloc_e5c8cb2c1eabdb1bd09aac14328fb50f, ptr %_0, align 4, !dbg !2398
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2398
  store i32 142, ptr %3, align 4, !dbg !2398
  br label %bb5, !dbg !2398

bb5:                                              ; preds = %bb2, %bb3, %bb4
  %4 = load ptr, ptr %_0, align 4, !dbg !2399
  %5 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2399
  %6 = load i32, ptr %5, align 4, !dbg !2399
  %7 = insertvalue { ptr, i32 } poison, ptr %4, 0, !dbg !2399
  %8 = insertvalue { ptr, i32 } %7, i32 %6, 1, !dbg !2399
  ret { ptr, i32 } %8, !dbg !2399
}

; wasi::lib_generated::fd_advise
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated9fd_advise17h93e0fb2b560d0ca7E(i32 %fd, i64 %offset, i64 %len, i8 %advice) unnamed_addr #2 !dbg !2400 {
start:
  %ret.dbg.spill = alloca [4 x i8], align 4
  %advice.dbg.spill = alloca [1 x i8], align 1
  %len.dbg.spill = alloca [8 x i8], align 8
  %offset.dbg.spill = alloca [8 x i8], align 8
  %fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2404, !DIExpression(), !2410)
  store i64 %offset, ptr %offset.dbg.spill, align 8
    #dbg_declare(ptr %offset.dbg.spill, !2405, !DIExpression(), !2411)
  store i64 %len, ptr %len.dbg.spill, align 8
    #dbg_declare(ptr %len.dbg.spill, !2406, !DIExpression(), !2412)
  store i8 %advice, ptr %advice.dbg.spill, align 1
    #dbg_declare(ptr %advice.dbg.spill, !2407, !DIExpression(), !2413)
  %_9 = zext i8 %advice to i32, !dbg !2414
; call wasi::lib_generated::wasi_snapshot_preview1::fd_advise
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19fd_advise17he82b97dc19668ec9E(i32 %fd, i64 %offset, i64 %len, i32 %_9) #52, !dbg !2415
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2415
    #dbg_declare(ptr %ret.dbg.spill, !2408, !DIExpression(), !2416)
  %0 = icmp eq i32 %ret, 0, !dbg !2417
  br i1 %0, label %bb3, label %bb2, !dbg !2417

bb3:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !2418
  br label %bb4, !dbg !2419

bb2:                                              ; preds = %start
  %_12 = trunc i32 %ret to i16, !dbg !2420
  %1 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2421
  store i16 %_12, ptr %1, align 2, !dbg !2421
  store i16 1, ptr %_0, align 2, !dbg !2421
  br label %bb4, !dbg !2422

bb4:                                              ; preds = %bb2, %bb3
  %2 = load i16, ptr %_0, align 2, !dbg !2423
  %3 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2423
  %4 = load i16, ptr %3, align 2, !dbg !2423
  %5 = insertvalue { i16, i16 } poison, i16 %2, 0, !dbg !2423
  %6 = insertvalue { i16, i16 } %5, i16 %4, 1, !dbg !2423
  ret { i16, i16 } %6, !dbg !2423
}

; wasi::lib_generated::fd_pwrite
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated9fd_pwrite17ha5ca9f0b139d9c68E(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 4 %iovs.0, i32 %iovs.1, i64 %offset) unnamed_addr #2 !dbg !2424 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %offset.dbg.spill = alloca [8 x i8], align 8
  %iovs.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2428, !DIExpression(), !2435)
  store ptr %iovs.0, ptr %iovs.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %iovs.dbg.spill, i32 4
  store i32 %iovs.1, ptr %0, align 4
    #dbg_declare(ptr %iovs.dbg.spill, !2429, !DIExpression(), !2436)
  store i64 %offset, ptr %offset.dbg.spill, align 8
    #dbg_declare(ptr %offset.dbg.spill, !2430, !DIExpression(), !2437)
    #dbg_declare(ptr %rp0, !2431, !DIExpression(), !2438)
  store i32 undef, ptr %rp0, align 4, !dbg !2439
  store ptr %iovs.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %iovs.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2339, !DIExpression(), !2440)
  %_7 = ptrtoint ptr %iovs.0 to i32, !dbg !2442
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2443)
  %_12 = ptrtoint ptr %rp0 to i32, !dbg !2445
; call wasi::lib_generated::wasi_snapshot_preview1::fd_pwrite
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19fd_pwrite17h559c5b74504703ddE(i32 %fd, i32 %_7, i32 %iovs.1, i64 %offset, i32 %_12) #52, !dbg !2446
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2446
    #dbg_declare(ptr %ret.dbg.spill, !2433, !DIExpression(), !2447)
  %2 = icmp eq i32 %ret, 0, !dbg !2448
  br i1 %2, label %bb6, label %bb5, !dbg !2448

bb6:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2449)
  %_17 = ptrtoint ptr %rp0 to i32, !dbg !2451
  %_16 = inttoptr i32 %_17 to ptr, !dbg !2451
; call core::ptr::read
  %_15 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_16, ptr align 4 @alloc_104041ffccf6d4e22976d0f9e6abad2d) #52, !dbg !2452
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2453
  store i32 %_15, ptr %3, align 4, !dbg !2453
  store i16 0, ptr %_0, align 4, !dbg !2453
  br label %bb9, !dbg !2454

bb5:                                              ; preds = %start
  %_21 = trunc i32 %ret to i16, !dbg !2455
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2456
  store i16 %_21, ptr %4, align 2, !dbg !2456
  store i16 1, ptr %_0, align 4, !dbg !2456
  br label %bb9, !dbg !2457

bb9:                                              ; preds = %bb5, %bb6
  ret void, !dbg !2458
}

; wasi::lib_generated::path_link
; Function Attrs: nounwind
define dso_local { i16, i16 } @_ZN4wasi13lib_generated9path_link17h2269c99aa733f901E(i32 %old_fd, i32 %old_flags, ptr align 1 %old_path.0, i32 %old_path.1, i32 %new_fd, ptr align 1 %new_path.0, i32 %new_path.1) unnamed_addr #2 !dbg !2459 {
start:
  %self.dbg.spill.i1 = alloca [8 x i8], align 4
  %self.dbg.spill.i = alloca [8 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %new_path.dbg.spill = alloca [8 x i8], align 4
  %new_fd.dbg.spill = alloca [4 x i8], align 4
  %old_path.dbg.spill = alloca [8 x i8], align 4
  %old_flags.dbg.spill = alloca [4 x i8], align 4
  %old_fd.dbg.spill = alloca [4 x i8], align 4
  %_0 = alloca [4 x i8], align 2
  store i32 %old_fd, ptr %old_fd.dbg.spill, align 4
    #dbg_declare(ptr %old_fd.dbg.spill, !2463, !DIExpression(), !2470)
  store i32 %old_flags, ptr %old_flags.dbg.spill, align 4
    #dbg_declare(ptr %old_flags.dbg.spill, !2464, !DIExpression(), !2471)
  store ptr %old_path.0, ptr %old_path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %old_path.dbg.spill, i32 4
  store i32 %old_path.1, ptr %0, align 4
    #dbg_declare(ptr %old_path.dbg.spill, !2465, !DIExpression(), !2472)
  store i32 %new_fd, ptr %new_fd.dbg.spill, align 4
    #dbg_declare(ptr %new_fd.dbg.spill, !2466, !DIExpression(), !2473)
  store ptr %new_path.0, ptr %new_path.dbg.spill, align 4
  %1 = getelementptr inbounds i8, ptr %new_path.dbg.spill, i32 4
  store i32 %new_path.1, ptr %1, align 4
    #dbg_declare(ptr %new_path.dbg.spill, !2467, !DIExpression(), !2474)
  store ptr %old_path.0, ptr %self.dbg.spill.i1, align 4
  %2 = getelementptr inbounds i8, ptr %self.dbg.spill.i1, i32 4
  store i32 %old_path.1, ptr %2, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !843, !DIExpression(), !2475)
  %_9 = ptrtoint ptr %old_path.0 to i32, !dbg !2477
; call core::str::<impl str>::len
  %_12 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %old_path.0, i32 %old_path.1) #52, !dbg !2478
  store ptr %new_path.0, ptr %self.dbg.spill.i, align 4
  %3 = getelementptr inbounds i8, ptr %self.dbg.spill.i, i32 4
  store i32 %new_path.1, ptr %3, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !843, !DIExpression(), !2479)
  %_14 = ptrtoint ptr %new_path.0 to i32, !dbg !2481
; call core::str::<impl str>::len
  %_17 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %new_path.0, i32 %new_path.1) #52, !dbg !2482
; call wasi::lib_generated::wasi_snapshot_preview1::path_link
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19path_link17hda477598ace46fc0E(i32 %old_fd, i32 %old_flags, i32 %_9, i32 %_12, i32 %new_fd, i32 %_14, i32 %_17) #52, !dbg !2483
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2483
    #dbg_declare(ptr %ret.dbg.spill, !2468, !DIExpression(), !2484)
  %4 = icmp eq i32 %ret, 0, !dbg !2485
  br i1 %4, label %bb7, label %bb6, !dbg !2485

bb7:                                              ; preds = %start
  store i16 0, ptr %_0, align 2, !dbg !2486
  br label %bb8, !dbg !2487

bb6:                                              ; preds = %start
  %_19 = trunc i32 %ret to i16, !dbg !2488
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2489
  store i16 %_19, ptr %5, align 2, !dbg !2489
  store i16 1, ptr %_0, align 2, !dbg !2489
  br label %bb8, !dbg !2490

bb8:                                              ; preds = %bb6, %bb7
  %6 = load i16, ptr %_0, align 2, !dbg !2491
  %7 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2491
  %8 = load i16, ptr %7, align 2, !dbg !2491
  %9 = insertvalue { i16, i16 } poison, i16 %6, 0, !dbg !2491
  %10 = insertvalue { i16, i16 } %9, i16 %8, 1, !dbg !2491
  ret { i16, i16 } %10, !dbg !2491
}

; wasi::lib_generated::path_open
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated9path_open17hffe6bc4ef505fe1eE(ptr sret([8 x i8]) align 4 %_0, i32 %fd, i32 %dirflags, ptr align 1 %path.0, i32 %path.1, i16 %oflags, i64 %fs_rights_base, i64 %fs_rights_inheriting, i16 %fdflags) unnamed_addr #2 !dbg !2492 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %fdflags.dbg.spill = alloca [2 x i8], align 2
  %fs_rights_inheriting.dbg.spill = alloca [8 x i8], align 8
  %fs_rights_base.dbg.spill = alloca [8 x i8], align 8
  %oflags.dbg.spill = alloca [2 x i8], align 2
  %path.dbg.spill = alloca [8 x i8], align 4
  %dirflags.dbg.spill = alloca [4 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2496, !DIExpression(), !2507)
  store i32 %dirflags, ptr %dirflags.dbg.spill, align 4
    #dbg_declare(ptr %dirflags.dbg.spill, !2497, !DIExpression(), !2508)
  store ptr %path.0, ptr %path.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %path.dbg.spill, i32 4
  store i32 %path.1, ptr %0, align 4
    #dbg_declare(ptr %path.dbg.spill, !2498, !DIExpression(), !2509)
  store i16 %oflags, ptr %oflags.dbg.spill, align 2
    #dbg_declare(ptr %oflags.dbg.spill, !2499, !DIExpression(), !2510)
  store i64 %fs_rights_base, ptr %fs_rights_base.dbg.spill, align 8
    #dbg_declare(ptr %fs_rights_base.dbg.spill, !2500, !DIExpression(), !2511)
  store i64 %fs_rights_inheriting, ptr %fs_rights_inheriting.dbg.spill, align 8
    #dbg_declare(ptr %fs_rights_inheriting.dbg.spill, !2501, !DIExpression(), !2512)
  store i16 %fdflags, ptr %fdflags.dbg.spill, align 2
    #dbg_declare(ptr %fdflags.dbg.spill, !2502, !DIExpression(), !2513)
    #dbg_declare(ptr %rp0, !2503, !DIExpression(), !2514)
  store i32 undef, ptr %rp0, align 4, !dbg !2515
  store ptr %path.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %path.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !843, !DIExpression(), !2516)
  %_12 = ptrtoint ptr %path.0 to i32, !dbg !2518
; call core::str::<impl str>::len
  %_15 = call i32 @"_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E"(ptr align 1 %path.0, i32 %path.1) #52, !dbg !2519
  %_16 = zext i16 %oflags to i32, !dbg !2520
  %_19 = zext i16 %fdflags to i32, !dbg !2521
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !991, !DIExpression(), !2522)
  %_20 = ptrtoint ptr %rp0 to i32, !dbg !2524
; call wasi::lib_generated::wasi_snapshot_preview1::path_open
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17hed24d4b5adffec4dE(i32 %fd, i32 %dirflags, i32 %_12, i32 %_15, i32 %_16, i64 %fs_rights_base, i64 %fs_rights_inheriting, i32 %_19, i32 %_20) #52, !dbg !2525
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2525
    #dbg_declare(ptr %ret.dbg.spill, !2505, !DIExpression(), !2526)
  %2 = icmp eq i32 %ret, 0, !dbg !2527
  br i1 %2, label %bb7, label %bb6, !dbg !2527

bb7:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !991, !DIExpression(), !2528)
  %_25 = ptrtoint ptr %rp0 to i32, !dbg !2530
  %_24 = inttoptr i32 %_25 to ptr, !dbg !2530
; call core::ptr::read
  %_23 = call i32 @_ZN4core3ptr4read17he4d71e30ba8af448E(ptr %_24, ptr align 4 @alloc_5cc1963d30ab6f24862bf937c52293bd) #52, !dbg !2531
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2532
  store i32 %_23, ptr %3, align 4, !dbg !2532
  store i16 0, ptr %_0, align 4, !dbg !2532
  br label %bb10, !dbg !2533

bb6:                                              ; preds = %start
  %_29 = trunc i32 %ret to i16, !dbg !2534
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2535
  store i16 %_29, ptr %4, align 2, !dbg !2535
  store i16 1, ptr %_0, align 4, !dbg !2535
  br label %bb10, !dbg !2536

bb10:                                             ; preds = %bb6, %bb7
  ret void, !dbg !2537
}

; wasi::lib_generated::proc_exit
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated9proc_exit17h6ac6fed0d1947302E(i32 %rval) unnamed_addr #2 !dbg !2538 {
start:
  %rval.dbg.spill = alloca [4 x i8], align 4
  store i32 %rval, ptr %rval.dbg.spill, align 4
    #dbg_declare(ptr %rval.dbg.spill, !2542, !DIExpression(), !2543)
; call wasi::lib_generated::wasi_snapshot_preview1::proc_exit
  call void @_ZN4wasi13lib_generated22wasi_snapshot_preview19proc_exit17h9c55f38a707d9ddaE(i32 %rval) #53, !dbg !2544
  unreachable, !dbg !2544
}

; wasi::lib_generated::sock_recv
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated9sock_recv17h8f741338cb57a0c4E(ptr sret([12 x i8]) align 4 %_0, i32 %fd, ptr align 4 %ri_data.0, i32 %ri_data.1, i16 %ri_flags) unnamed_addr #2 !dbg !2545 {
start:
  %self.dbg.spill.i4 = alloca [8 x i8], align 4
  %self.dbg.spill.i3 = alloca [4 x i8], align 4
  %self.dbg.spill.i2 = alloca [4 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %ri_flags.dbg.spill = alloca [2 x i8], align 2
  %ri_data.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp1 = alloca [2 x i8], align 2
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2568, !DIExpression(), !2584)
  store ptr %ri_data.0, ptr %ri_data.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %ri_data.dbg.spill, i32 4
  store i32 %ri_data.1, ptr %0, align 4
    #dbg_declare(ptr %ri_data.dbg.spill, !2569, !DIExpression(), !2585)
  store i16 %ri_flags, ptr %ri_flags.dbg.spill, align 2
    #dbg_declare(ptr %ri_flags.dbg.spill, !2570, !DIExpression(), !2586)
    #dbg_declare(ptr %rp0, !2571, !DIExpression(), !2587)
    #dbg_declare(ptr %rp1, !2573, !DIExpression(), !2588)
  store i32 undef, ptr %rp0, align 4, !dbg !2589
  store i16 undef, ptr %rp1, align 2, !dbg !2590
  store ptr %ri_data.0, ptr %self.dbg.spill.i4, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i4, i32 4
  store i32 %ri_data.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i4, !2096, !DIExpression(), !2591)
  %_8 = ptrtoint ptr %ri_data.0 to i32, !dbg !2593
  %_12 = zext i16 %ri_flags to i32, !dbg !2594
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2595)
  %_13 = ptrtoint ptr %rp0 to i32, !dbg !2597
  store ptr %rp1, ptr %self.dbg.spill.i3, align 4
    #dbg_declare(ptr %self.dbg.spill.i3, !2598, !DIExpression(), !2606)
  %_16 = ptrtoint ptr %rp1 to i32, !dbg !2608
; call wasi::lib_generated::wasi_snapshot_preview1::sock_recv
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19sock_recv17h7cbade705e685438E(i32 %fd, i32 %_8, i32 %ri_data.1, i32 %_12, i32 %_13, i32 %_16) #52, !dbg !2609
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2609
    #dbg_declare(ptr %ret.dbg.spill, !2582, !DIExpression(), !2610)
  %2 = icmp eq i32 %ret, 0, !dbg !2611
  br i1 %2, label %bb8, label %bb7, !dbg !2611

bb8:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2612)
  %_22 = ptrtoint ptr %rp0 to i32, !dbg !2614
  %_21 = inttoptr i32 %_22 to ptr, !dbg !2614
; call core::ptr::read
  %_20 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_21, ptr align 4 @alloc_d1d574f28c4f5458a7bfb16314c7c105) #52, !dbg !2615
  store ptr %rp1, ptr %self.dbg.spill.i2, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2598, !DIExpression(), !2616)
  %_27 = ptrtoint ptr %rp1 to i32, !dbg !2618
  %_26 = inttoptr i32 %_27 to ptr, !dbg !2618
; call core::ptr::read
  %_25 = call i16 @_ZN4core3ptr4read17h188bb76cc8094201E(ptr %_26, ptr align 4 @alloc_adde0fd5d4a3a7ff0a32f004052d93e0) #52, !dbg !2619
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2620
  store i32 %_20, ptr %3, align 4, !dbg !2620
  %4 = getelementptr inbounds i8, ptr %3, i32 4, !dbg !2620
  store i16 %_25, ptr %4, align 4, !dbg !2620
  store i16 0, ptr %_0, align 4, !dbg !2620
  br label %bb13, !dbg !2621

bb7:                                              ; preds = %start
  %_31 = trunc i32 %ret to i16, !dbg !2622
  %5 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2623
  store i16 %_31, ptr %5, align 2, !dbg !2623
  store i16 1, ptr %_0, align 4, !dbg !2623
  br label %bb13, !dbg !2624

bb13:                                             ; preds = %bb7, %bb8
  ret void, !dbg !2625
}

; wasi::lib_generated::sock_send
; Function Attrs: nounwind
define dso_local void @_ZN4wasi13lib_generated9sock_send17h88a407874ec40b2fE(ptr sret([8 x i8]) align 4 %_0, i32 %fd, ptr align 4 %si_data.0, i32 %si_data.1, i16 %si_flags) unnamed_addr #2 !dbg !2626 {
start:
  %self.dbg.spill.i2 = alloca [8 x i8], align 4
  %self.dbg.spill.i1 = alloca [4 x i8], align 4
  %self.dbg.spill.i = alloca [4 x i8], align 4
  %ret.dbg.spill = alloca [4 x i8], align 4
  %si_flags.dbg.spill = alloca [2 x i8], align 2
  %si_data.dbg.spill = alloca [8 x i8], align 4
  %fd.dbg.spill = alloca [4 x i8], align 4
  %rp0 = alloca [4 x i8], align 4
  store i32 %fd, ptr %fd.dbg.spill, align 4
    #dbg_declare(ptr %fd.dbg.spill, !2630, !DIExpression(), !2637)
  store ptr %si_data.0, ptr %si_data.dbg.spill, align 4
  %0 = getelementptr inbounds i8, ptr %si_data.dbg.spill, i32 4
  store i32 %si_data.1, ptr %0, align 4
    #dbg_declare(ptr %si_data.dbg.spill, !2631, !DIExpression(), !2638)
  store i16 %si_flags, ptr %si_flags.dbg.spill, align 2
    #dbg_declare(ptr %si_flags.dbg.spill, !2632, !DIExpression(), !2639)
    #dbg_declare(ptr %rp0, !2633, !DIExpression(), !2640)
  store i32 undef, ptr %rp0, align 4, !dbg !2641
  store ptr %si_data.0, ptr %self.dbg.spill.i2, align 4
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill.i2, i32 4
  store i32 %si_data.1, ptr %1, align 4
    #dbg_declare(ptr %self.dbg.spill.i2, !2339, !DIExpression(), !2642)
  %_7 = ptrtoint ptr %si_data.0 to i32, !dbg !2644
  %_11 = zext i16 %si_flags to i32, !dbg !2645
  store ptr %rp0, ptr %self.dbg.spill.i1, align 4
    #dbg_declare(ptr %self.dbg.spill.i1, !639, !DIExpression(), !2646)
  %_12 = ptrtoint ptr %rp0 to i32, !dbg !2648
; call wasi::lib_generated::wasi_snapshot_preview1::sock_send
  %ret = call i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19sock_send17hf406bd013fe8b827E(i32 %fd, i32 %_7, i32 %si_data.1, i32 %_11, i32 %_12) #52, !dbg !2649
  store i32 %ret, ptr %ret.dbg.spill, align 4, !dbg !2649
    #dbg_declare(ptr %ret.dbg.spill, !2635, !DIExpression(), !2650)
  %2 = icmp eq i32 %ret, 0, !dbg !2651
  br i1 %2, label %bb6, label %bb5, !dbg !2651

bb6:                                              ; preds = %start
  store ptr %rp0, ptr %self.dbg.spill.i, align 4
    #dbg_declare(ptr %self.dbg.spill.i, !639, !DIExpression(), !2652)
  %_17 = ptrtoint ptr %rp0 to i32, !dbg !2654
  %_16 = inttoptr i32 %_17 to ptr, !dbg !2654
; call core::ptr::read
  %_15 = call i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr %_16, ptr align 4 @alloc_727bdb16db3c13deb29d24e6ce1d0490) #52, !dbg !2655
  %3 = getelementptr inbounds i8, ptr %_0, i32 4, !dbg !2656
  store i32 %_15, ptr %3, align 4, !dbg !2656
  store i16 0, ptr %_0, align 4, !dbg !2656
  br label %bb9, !dbg !2657

bb5:                                              ; preds = %start
  %_21 = trunc i32 %ret to i16, !dbg !2658
  %4 = getelementptr inbounds i8, ptr %_0, i32 2, !dbg !2659
  store i16 %_21, ptr %4, align 2, !dbg !2659
  store i16 1, ptr %_0, align 4, !dbg !2659
  br label %bb9, !dbg !2660

bb9:                                              ; preds = %bb5, %bb6
  ret void, !dbg !2661
}

; <wasi::lib_generated::Errno as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN63_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Debug$GT$3fmt17hefc646a6f106b0e7E"(ptr align 2 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2662 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2667, !DIExpression(), !2669)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2668, !DIExpression(), !2670)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_87c86a8fda32926a6ab8441e110c5b98, i32 5) #52, !dbg !2671
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.0) #52, !dbg !2672
; call wasi::lib_generated::Errno::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated5Errno4name17h3a1ad9514045c510E(ptr align 2 %self) #52, !dbg !2673
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2673
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2673
  store ptr %1, ptr %_15, align 4, !dbg !2673
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2673
  store i32 %2, ptr %3, align 4, !dbg !2673
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2674
; call wasi::lib_generated::Errno::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated5Errno7message17h331e22806c2b66acE(ptr align 2 %self) #52, !dbg !2675
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2675
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2675
  store ptr %5, ptr %_19, align 4, !dbg !2675
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2675
  store i32 %6, ptr %7, align 4, !dbg !2675
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2676
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2677
  ret i1 %_0, !dbg !2678
}

; <wasi::lib_generated::Advice as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN64_$LT$wasi..lib_generated..Advice$u20$as$u20$core..fmt..Debug$GT$3fmt17h269077da97230a8cE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2679 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2684, !DIExpression(), !2686)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2685, !DIExpression(), !2687)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_969ebbff1873a839222fe5aabe10ba14, i32 6) #52, !dbg !2688
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2689
; call wasi::lib_generated::Advice::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated6Advice4name17h144c54f5ff34b5e2E(ptr align 1 %self) #52, !dbg !2690
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2690
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2690
  store ptr %1, ptr %_15, align 4, !dbg !2690
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2690
  store i32 %2, ptr %3, align 4, !dbg !2690
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2691
; call wasi::lib_generated::Advice::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated6Advice7message17h7a17fd969790b8c7E(ptr align 1 %self) #52, !dbg !2692
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2692
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2692
  store ptr %5, ptr %_19, align 4, !dbg !2692
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2692
  store i32 %6, ptr %7, align 4, !dbg !2692
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2693
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2694
  ret i1 %_0, !dbg !2695
}

; <wasi::lib_generated::Signal as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN64_$LT$wasi..lib_generated..Signal$u20$as$u20$core..fmt..Debug$GT$3fmt17ha37f77817edb24beE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2696 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2701, !DIExpression(), !2703)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2702, !DIExpression(), !2704)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_dc3191724e59b325c68c4dafbe9e2a7b, i32 6) #52, !dbg !2705
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2706
; call wasi::lib_generated::Signal::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated6Signal4name17h44b7507ff21226fcE(ptr align 1 %self) #52, !dbg !2707
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2707
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2707
  store ptr %1, ptr %_15, align 4, !dbg !2707
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2707
  store i32 %2, ptr %3, align 4, !dbg !2707
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2708
; call wasi::lib_generated::Signal::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated6Signal7message17h5c961ce5e1cd5f15E(ptr align 1 %self) #52, !dbg !2709
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2709
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2709
  store ptr %5, ptr %_19, align 4, !dbg !2709
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2709
  store i32 %6, ptr %7, align 4, !dbg !2709
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2710
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2711
  ret i1 %_0, !dbg !2712
}

; <wasi::lib_generated::Whence as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN64_$LT$wasi..lib_generated..Whence$u20$as$u20$core..fmt..Debug$GT$3fmt17h2c6ea1883e0a7c8eE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2713 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2718, !DIExpression(), !2720)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2719, !DIExpression(), !2721)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_e6d527c7a091e34b073b090f1620dcda, i32 6) #52, !dbg !2722
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2723
; call wasi::lib_generated::Whence::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated6Whence4name17he958b0be86dcb2e2E(ptr align 1 %self) #52, !dbg !2724
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2724
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2724
  store ptr %1, ptr %_15, align 4, !dbg !2724
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2724
  store i32 %2, ptr %3, align 4, !dbg !2724
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2725
; call wasi::lib_generated::Whence::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated6Whence7message17ha9e612d937001f4fE(ptr align 1 %self) #52, !dbg !2726
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2726
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2726
  store ptr %5, ptr %_19, align 4, !dbg !2726
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2726
  store i32 %6, ptr %7, align 4, !dbg !2726
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2727
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2728
  ret i1 %_0, !dbg !2729
}

; <wasi::lib_generated::Clockid as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN65_$LT$wasi..lib_generated..Clockid$u20$as$u20$core..fmt..Debug$GT$3fmt17h99a1a83dc5fa5389E"(ptr align 4 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2730 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2735, !DIExpression(), !2737)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2736, !DIExpression(), !2738)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_fd43fcb84d193089e2ff9355db271b4d, i32 7) #52, !dbg !2739
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.3) #52, !dbg !2740
; call wasi::lib_generated::Clockid::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated7Clockid4name17h908f75bcb5817523E(ptr align 4 %self) #52, !dbg !2741
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2741
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2741
  store ptr %1, ptr %_15, align 4, !dbg !2741
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2741
  store i32 %2, ptr %3, align 4, !dbg !2741
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2742
; call wasi::lib_generated::Clockid::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated7Clockid7message17hf15037538852fe88E(ptr align 4 %self) #52, !dbg !2743
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2743
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2743
  store ptr %5, ptr %_19, align 4, !dbg !2743
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2743
  store i32 %6, ptr %7, align 4, !dbg !2743
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2744
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2745
  ret i1 %_0, !dbg !2746
}

; <wasi::lib_generated::Errno as core::fmt::Display>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN65_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Display$GT$3fmt17hd6a85332ea0c6f1aE"(ptr align 2 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2747 {
start:
  %args.dbg.spill = alloca [8 x i8], align 4
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_10 = alloca [8 x i8], align 4
  %_9 = alloca [8 x i8], align 4
  %args = alloca [16 x i8], align 4
  %_6 = alloca [8 x i8], align 4
  %_3 = alloca [24 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2750, !DIExpression(), !2766)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2751, !DIExpression(), !2767)
    #dbg_declare(ptr %args, !2760, !DIExpression(), !2768)
; call wasi::lib_generated::Errno::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated5Errno4name17h3a1ad9514045c510E(ptr align 2 %self) #52, !dbg !2769
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2769
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2769
  store ptr %1, ptr %_6, align 4, !dbg !2769
  %3 = getelementptr inbounds i8, ptr %_6, i32 4, !dbg !2769
  store i32 %2, ptr %3, align 4, !dbg !2769
  store ptr %_6, ptr %args.dbg.spill, align 4, !dbg !2770
  %4 = getelementptr inbounds i8, ptr %args.dbg.spill, i32 4, !dbg !2770
  store ptr %self, ptr %4, align 4, !dbg !2770
    #dbg_declare(ptr %args.dbg.spill, !2752, !DIExpression(), !2771)
; call core::fmt::rt::Argument::new_display
  call void @_ZN4core3fmt2rt8Argument11new_display17h50897f56d49ff652E(ptr sret([8 x i8]) align 4 %_9, ptr align 4 %_6) #52, !dbg !2771
; call core::fmt::rt::Argument::new_display
  call void @_ZN4core3fmt2rt8Argument11new_display17hec09264c35894736E(ptr sret([8 x i8]) align 4 %_10, ptr align 2 %self) #52, !dbg !2771
  %5 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args, i32 0, !dbg !2771
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %5, ptr align 4 %_9, i32 8, i1 false), !dbg !2771
  %6 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args, i32 1, !dbg !2771
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %6, ptr align 4 %_10, i32 8, i1 false), !dbg !2771
; call core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hc865914cb945b354E"(ptr sret([24 x i8]) align 4 %_3, ptr align 4 @alloc_c6c92eab63b644b2c14141e96344f22f, ptr align 4 %args) #52, !dbg !2768
; call core::fmt::Formatter::write_fmt
  %_0 = call zeroext i1 @_ZN4core3fmt9Formatter9write_fmt17h5e1a4779fbbec593E(ptr align 4 %f, ptr align 4 %_3) #52, !dbg !2770
  ret i1 %_0, !dbg !2772
}

; <wasi::lib_generated::Filetype as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN66_$LT$wasi..lib_generated..Filetype$u20$as$u20$core..fmt..Debug$GT$3fmt17hd766654a1d63f04fE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2773 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2778, !DIExpression(), !2780)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2779, !DIExpression(), !2781)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_19726b29c768e359727fb4780e161989, i32 8) #52, !dbg !2782
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2783
; call wasi::lib_generated::Filetype::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated8Filetype4name17h42a52d431e4e0e3fE(ptr align 1 %self) #52, !dbg !2784
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2784
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2784
  store ptr %1, ptr %_15, align 4, !dbg !2784
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2784
  store i32 %2, ptr %3, align 4, !dbg !2784
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2785
; call wasi::lib_generated::Filetype::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated8Filetype7message17hc28156d203146fb1E(ptr align 1 %self) #52, !dbg !2786
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2786
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2786
  store ptr %5, ptr %_19, align 4, !dbg !2786
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2786
  store i32 %6, ptr %7, align 4, !dbg !2786
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2787
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2788
  ret i1 %_0, !dbg !2789
}

; <wasi::lib_generated::Eventtype as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN67_$LT$wasi..lib_generated..Eventtype$u20$as$u20$core..fmt..Debug$GT$3fmt17h1543bb8c0ad4c88aE"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2790 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2795, !DIExpression(), !2797)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2796, !DIExpression(), !2798)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_56edaa5d846bdfc37ea25102366e3bbb, i32 9) #52, !dbg !2799
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2800
; call wasi::lib_generated::Eventtype::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated9Eventtype4name17h6a88a0e670107d50E(ptr align 1 %self) #52, !dbg !2801
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2801
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2801
  store ptr %1, ptr %_15, align 4, !dbg !2801
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2801
  store i32 %2, ptr %3, align 4, !dbg !2801
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2802
; call wasi::lib_generated::Eventtype::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated9Eventtype7message17h94566f151fe315c9E(ptr align 1 %self) #52, !dbg !2803
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2803
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2803
  store ptr %5, ptr %_19, align 4, !dbg !2803
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2803
  store i32 %6, ptr %7, align 4, !dbg !2803
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2804
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2805
  ret i1 %_0, !dbg !2806
}

; <wasi::lib_generated::Preopentype as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define dso_local zeroext i1 @"_ZN69_$LT$wasi..lib_generated..Preopentype$u20$as$u20$core..fmt..Debug$GT$3fmt17h0d6b43c88f2d34b4E"(ptr align 1 %self, ptr align 4 %f) unnamed_addr #2 !dbg !2807 {
start:
  %f.dbg.spill = alloca [4 x i8], align 4
  %self.dbg.spill = alloca [4 x i8], align 4
  %_19 = alloca [8 x i8], align 4
  %_15 = alloca [8 x i8], align 4
  %_7 = alloca [8 x i8], align 4
  store ptr %self, ptr %self.dbg.spill, align 4
    #dbg_declare(ptr %self.dbg.spill, !2812, !DIExpression(), !2814)
  store ptr %f, ptr %f.dbg.spill, align 4
    #dbg_declare(ptr %f.dbg.spill, !2813, !DIExpression(), !2815)
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4 %_7, ptr align 4 %f, ptr align 1 @alloc_227530437c014becb32b0bafc4302ba5, i32 11) #52, !dbg !2816
; call core::fmt::builders::DebugStruct::field
  %_5 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_7, ptr align 1 @alloc_905976595ed1b08e57e2b44a2acadea4, i32 4, ptr align 1 %self, ptr align 4 @vtable.2) #52, !dbg !2817
; call wasi::lib_generated::Preopentype::name
  %0 = call { ptr, i32 } @_ZN4wasi13lib_generated11Preopentype4name17h51a1ca3954e9478cE(ptr align 1 %self) #52, !dbg !2818
  %1 = extractvalue { ptr, i32 } %0, 0, !dbg !2818
  %2 = extractvalue { ptr, i32 } %0, 1, !dbg !2818
  store ptr %1, ptr %_15, align 4, !dbg !2818
  %3 = getelementptr inbounds i8, ptr %_15, i32 4, !dbg !2818
  store i32 %2, ptr %3, align 4, !dbg !2818
; call core::fmt::builders::DebugStruct::field
  %_4 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_5, ptr align 1 @alloc_f00db71d77c58f05d86c38101680e143, i32 4, ptr align 1 %_15, ptr align 4 @vtable.1) #52, !dbg !2819
; call wasi::lib_generated::Preopentype::message
  %4 = call { ptr, i32 } @_ZN4wasi13lib_generated11Preopentype7message17ha683a318ca855ec8E(ptr align 1 %self) #52, !dbg !2820
  %5 = extractvalue { ptr, i32 } %4, 0, !dbg !2820
  %6 = extractvalue { ptr, i32 } %4, 1, !dbg !2820
  store ptr %5, ptr %_19, align 4, !dbg !2820
  %7 = getelementptr inbounds i8, ptr %_19, i32 4, !dbg !2820
  store i32 %6, ptr %7, align 4, !dbg !2820
; call core::fmt::builders::DebugStruct::field
  %_3 = call align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4 %_4, ptr align 1 @alloc_96af468510ea8f5f9cb1c5ccd138c101, i32 7, ptr align 1 %_19, ptr align 4 @vtable.1) #52, !dbg !2821
; call core::fmt::builders::DebugStruct::finish
  %_0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4 %_3) #52, !dbg !2822
  ret i1 %_0, !dbg !2823
}

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h72b54bf2b5971ea0E(ptr align 4) unnamed_addr #2

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17hda8089ad17629515E(ptr align 4) unnamed_addr #2

; core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf2721191f040b59aE"(ptr align 1, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::UpperHex for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h3a28df7a448ec4e8E"(ptr align 1, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::LowerHex for u8>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hf19d67508dd58d4aE"(ptr align 1, ptr align 4) unnamed_addr #2

; core::fmt::num::imp::<impl core::fmt::Display for u16>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u16$GT$3fmt17hbe9cdadc62270c33E"(ptr align 2, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::UpperHex for u16>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u16$GT$3fmt17hf846c66d16667c7bE"(ptr align 2, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::LowerHex for u16>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u16$GT$3fmt17hfc6eebde7cab5daeE"(ptr align 2, ptr align 4) unnamed_addr #2

; core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hd4bf219ffc62a5f7E"(ptr align 4, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::UpperHex for u32>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17he005099c42d93007E"(ptr align 4, ptr align 4) unnamed_addr #2

; core::fmt::num::<impl core::fmt::LowerHex for u32>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17hc2f3e52917b39c8dE"(ptr align 4, ptr align 4) unnamed_addr #2

; core::option::Option<T>::is_some
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core6option15Option$LT$T$GT$7is_some17h59be6c2f34b18cc5E"(ptr align 4) unnamed_addr #0

; Function Attrs: convergent nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.is.constant.i1(i1) #3

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #4

; core::fmt::write
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt5write17hc2899684fc6bf93bE(ptr align 1, ptr align 4, ptr align 4) unnamed_addr #2

; core::fmt::rt::<impl core::fmt::Arguments>::new_const
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$9new_const17hcd893c7b336178b4E"(ptr sret([24 x i8]) align 4, ptr align 4) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #5

; core::ptr::const_ptr::<impl *const T>::is_aligned_to
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13is_aligned_to17h9d68787d1567a990E"(ptr, i32) unnamed_addr #0

; core::ptr::const_ptr::<impl *const T>::is_null
; Function Attrs: inlinehint nounwind
declare dso_local zeroext i1 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17hf56eacc16313c5f5E"(ptr) unnamed_addr #0

; wasi::lib_generated::wasi_snapshot_preview1::fd_readdir
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110fd_readdir17hadee90ca69ca3b01E(i32, i32, i32, i64, i32) unnamed_addr #6

; core::ptr::read
; Function Attrs: inlinehint nounwind
declare dso_local i32 @_ZN4core3ptr4read17h78681311ce0dc199E(ptr, ptr align 4) unnamed_addr #0

; wasi::lib_generated::wasi_snapshot_preview1::proc_raise
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110proc_raise17h88509519ac6671ebE(i32) unnamed_addr #7

; wasi::lib_generated::wasi_snapshot_preview1::random_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview110random_get17h8b3b06eaef0413dfE(i32, i32) unnamed_addr #8

; wasi::lib_generated::wasi_snapshot_preview1::environ_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111environ_get17h8a9aabfc46a9fff9E(i32, i32) unnamed_addr #9

; wasi::lib_generated::wasi_snapshot_preview1::fd_allocate
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_allocate17h306cc9eead141b87E(i32, i64, i64) unnamed_addr #10

; wasi::lib_generated::wasi_snapshot_preview1::fd_datasync
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_datasync17h688f1e6b68c8337dE(i32) unnamed_addr #11

; wasi::lib_generated::wasi_snapshot_preview1::fd_renumber
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111fd_renumber17h347a31e5b77196e1E(i32, i32) unnamed_addr #12

; wasi::lib_generated::wasi_snapshot_preview1::path_rename
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111path_rename17h6a4b38a346375893E(i32, i32, i32, i32, i32, i32) unnamed_addr #13

; wasi::lib_generated::wasi_snapshot_preview1::poll_oneoff
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111poll_oneoff17h7ab1714565cdb552E(i32, i32, i32, i32) unnamed_addr #14

; wasi::lib_generated::wasi_snapshot_preview1::sched_yield
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111sched_yield17h2823555b32388fa4E() unnamed_addr #15

; wasi::lib_generated::wasi_snapshot_preview1::sock_accept
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview111sock_accept17h1fd793f808c8f883E(i32, i32, i32) unnamed_addr #16

; wasi::lib_generated::wasi_snapshot_preview1::path_symlink
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview112path_symlink17h19399a3353cf728fE(i32, i32, i32, i32, i32) unnamed_addr #17

; wasi::lib_generated::wasi_snapshot_preview1::clock_res_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113clock_res_get17h086a49b7a4f55e44E(i32, i32) unnamed_addr #18

; wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113fd_fdstat_get17h580d0524d5381556E(i32, i32) unnamed_addr #19

; wasi::lib_generated::wasi_snapshot_preview1::path_readlink
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113path_readlink17hc076c57d812bf0e0E(i32, i32, i32, i32, i32, i32) unnamed_addr #20

; wasi::lib_generated::wasi_snapshot_preview1::sock_shutdown
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview113sock_shutdown17h7589812e8ce2a8d2E(i32, i32) unnamed_addr #21

; wasi::lib_generated::wasi_snapshot_preview1::args_sizes_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114args_sizes_get17hf2eb9ec185cc8efeE(i32, i32) unnamed_addr #22

; wasi::lib_generated::wasi_snapshot_preview1::clock_time_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114clock_time_get17h4dc8e8bee1835d0cE(i32, i64, i32) unnamed_addr #23

; wasi::lib_generated::wasi_snapshot_preview1::fd_prestat_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview114fd_prestat_get17hc6fcf80e1c03dd6bE(i32, i32) unnamed_addr #24

; wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview115fd_filestat_get17h501df1c79e23b9ffE(i32, i32) unnamed_addr #25

; wasi::lib_generated::wasi_snapshot_preview1::path_unlink_file
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview116path_unlink_file17hacac4e766be441c4E(i32, i32, i32) unnamed_addr #26

; wasi::lib_generated::wasi_snapshot_preview1::environ_sizes_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview117environ_sizes_get17h3870c602c4670daeE(i32, i32) unnamed_addr #27

; wasi::lib_generated::wasi_snapshot_preview1::path_filestat_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview117path_filestat_get17h721971769c2cf374E(i32, i32, i32, i32, i32) unnamed_addr #28

; wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_set_flags
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview119fd_fdstat_set_flags17h1498b47897c0a35bE(i32, i32) unnamed_addr #29

; wasi::lib_generated::wasi_snapshot_preview1::fd_prestat_dir_name
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview119fd_prestat_dir_name17h7b043e4708de23a4E(i32, i32, i32) unnamed_addr #30

; wasi::lib_generated::wasi_snapshot_preview1::fd_fdstat_set_rights
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview120fd_fdstat_set_rights17h0c3f8a2925c7d2c0E(i32, i64, i64) unnamed_addr #31

; wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_set_size
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview120fd_filestat_set_size17h7ed0a5f28827e26aE(i32, i64) unnamed_addr #32

; wasi::lib_generated::wasi_snapshot_preview1::fd_filestat_set_times
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121fd_filestat_set_times17h6a0126c21b0af73dE(i32, i64, i64, i32) unnamed_addr #33

; wasi::lib_generated::wasi_snapshot_preview1::path_create_directory
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121path_create_directory17h043ddd360752c37aE(i32, i32, i32) unnamed_addr #34

; wasi::lib_generated::wasi_snapshot_preview1::path_remove_directory
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview121path_remove_directory17hdcdfe2f9bcd9fc06E(i32, i32, i32) unnamed_addr #35

; wasi::lib_generated::wasi_snapshot_preview1::path_filestat_set_times
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview123path_filestat_set_times17h49b1faeed91aeeccE(i32, i32, i32, i32, i64, i64, i32) unnamed_addr #36

; wasi::lib_generated::wasi_snapshot_preview1::fd_read
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_read17hb9636e7cb0eba53eE(i32, i32, i32, i32) unnamed_addr #37

; wasi::lib_generated::wasi_snapshot_preview1::fd_seek
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_seek17hc5a5c23fbf7b77adE(i32, i64, i32, i32) unnamed_addr #38

; wasi::lib_generated::wasi_snapshot_preview1::fd_sync
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_sync17h00da7a4290b09810E(i32) unnamed_addr #39

; wasi::lib_generated::wasi_snapshot_preview1::fd_tell
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview17fd_tell17hb423faf6c354b520E(i32, i32) unnamed_addr #40

; wasi::lib_generated::wasi_snapshot_preview1::args_get
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18args_get17haf4396b9fc3f17b1E(i32, i32) unnamed_addr #41

; wasi::lib_generated::wasi_snapshot_preview1::fd_close
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_close17he82b66267a7c17f3E(i32) unnamed_addr #42

; wasi::lib_generated::wasi_snapshot_preview1::fd_pread
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_pread17hcbdc8a7366acee58E(i32, i32, i32, i64, i32) unnamed_addr #43

; wasi::lib_generated::wasi_snapshot_preview1::fd_write
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17hdb524b0877e7bdf7E(i32, i32, i32, i32) unnamed_addr #44

; wasi::lib_generated::wasi_snapshot_preview1::fd_advise
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19fd_advise17he82b97dc19668ec9E(i32, i64, i64, i32) unnamed_addr #45

; wasi::lib_generated::wasi_snapshot_preview1::fd_pwrite
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19fd_pwrite17h559c5b74504703ddE(i32, i32, i32, i64, i32) unnamed_addr #46

; wasi::lib_generated::wasi_snapshot_preview1::path_link
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19path_link17hda477598ace46fc0E(i32, i32, i32, i32, i32, i32, i32) unnamed_addr #47

; wasi::lib_generated::wasi_snapshot_preview1::path_open
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17hed24d4b5adffec4dE(i32, i32, i32, i32, i32, i64, i64, i32, i32) unnamed_addr #48

; wasi::lib_generated::wasi_snapshot_preview1::proc_exit
; Function Attrs: noreturn nounwind
declare dso_local void @_ZN4wasi13lib_generated22wasi_snapshot_preview19proc_exit17h9c55f38a707d9ddaE(i32) unnamed_addr #49

; wasi::lib_generated::wasi_snapshot_preview1::sock_recv
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19sock_recv17h7cbade705e685438E(i32, i32, i32, i32, i32, i32) unnamed_addr #50

; wasi::lib_generated::wasi_snapshot_preview1::sock_send
; Function Attrs: nounwind
declare dso_local i32 @_ZN4wasi13lib_generated22wasi_snapshot_preview19sock_send17hf406bd013fe8b827E(i32, i32, i32, i32, i32) unnamed_addr #51

; core::fmt::Formatter::debug_struct
; Function Attrs: nounwind
declare dso_local void @_ZN4core3fmt9Formatter12debug_struct17h0347acd38896f337E(ptr sret([8 x i8]) align 4, ptr align 4, ptr align 1, i32) unnamed_addr #2

; core::fmt::builders::DebugStruct::field
; Function Attrs: nounwind
declare dso_local align 4 ptr @_ZN4core3fmt8builders11DebugStruct5field17h072b3079aa03c4c7E(ptr align 4, ptr align 1, i32, ptr align 1, ptr align 4) unnamed_addr #2

; <&T as core::fmt::Debug>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbffdc5d01bf5408bE"(ptr align 4, ptr align 4) unnamed_addr #2

; core::fmt::builders::DebugStruct::finish
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt8builders11DebugStruct6finish17h10d6526282454380E(ptr align 4) unnamed_addr #2

; core::fmt::rt::Argument::new_display
; Function Attrs: inlinehint nounwind
declare dso_local void @_ZN4core3fmt2rt8Argument11new_display17h50897f56d49ff652E(ptr sret([8 x i8]) align 4, ptr align 4) unnamed_addr #0

; core::fmt::rt::Argument::new_display
; Function Attrs: inlinehint nounwind
declare dso_local void @_ZN4core3fmt2rt8Argument11new_display17hec09264c35894736E(ptr sret([8 x i8]) align 4, ptr align 2) unnamed_addr #0

; core::fmt::rt::<impl core::fmt::Arguments>::new_v1
; Function Attrs: inlinehint nounwind
declare dso_local void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117hc865914cb945b354E"(ptr sret([24 x i8]) align 4, ptr align 4, ptr align 4) unnamed_addr #0

attributes #0 = { inlinehint nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #1 = { inlinehint noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #2 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" }
attributes #3 = { convergent nocallback nofree nosync nounwind willreturn memory(none) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #6 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_readdir" }
attributes #7 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="proc_raise" }
attributes #8 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="random_get" }
attributes #9 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="environ_get" }
attributes #10 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_allocate" }
attributes #11 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_datasync" }
attributes #12 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_renumber" }
attributes #13 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_rename" }
attributes #14 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="poll_oneoff" }
attributes #15 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="sched_yield" }
attributes #16 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="sock_accept" }
attributes #17 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_symlink" }
attributes #18 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="clock_res_get" }
attributes #19 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_fdstat_get" }
attributes #20 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_readlink" }
attributes #21 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="sock_shutdown" }
attributes #22 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="args_sizes_get" }
attributes #23 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="clock_time_get" }
attributes #24 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_prestat_get" }
attributes #25 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_filestat_get" }
attributes #26 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_unlink_file" }
attributes #27 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="environ_sizes_get" }
attributes #28 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_filestat_get" }
attributes #29 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_fdstat_set_flags" }
attributes #30 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_prestat_dir_name" }
attributes #31 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_fdstat_set_rights" }
attributes #32 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_filestat_set_size" }
attributes #33 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_filestat_set_times" }
attributes #34 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_create_directory" }
attributes #35 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_remove_directory" }
attributes #36 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_filestat_set_times" }
attributes #37 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_read" }
attributes #38 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_seek" }
attributes #39 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_sync" }
attributes #40 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_tell" }
attributes #41 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="args_get" }
attributes #42 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_close" }
attributes #43 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_pread" }
attributes #44 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_write" }
attributes #45 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_advise" }
attributes #46 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="fd_pwrite" }
attributes #47 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_link" }
attributes #48 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="path_open" }
attributes #49 = { noreturn nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="proc_exit" }
attributes #50 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="sock_recv" }
attributes #51 = { nounwind "target-cpu"="generic" "target-features"="+atomics,+bulk-memory,+mutable-globals" "wasm-import-module"="wasi_snapshot_preview1" "wasm-import-name"="sock_send" }
attributes #52 = { nounwind }
attributes #53 = { noreturn nounwind }

!llvm.ident = !{!45}
!llvm.dbg.cu = !{!46}
!llvm.module.flags = !{!49, !50}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<u16 as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<u16 as core::fmt::Debug>::{vtable_type}", file: !2, size: 128, align: 32, flags: DIFlagArtificial, elements: !4, vtableHolder: !12, templateParams: !13, identifier: "fffaff872192bedf9b7c17461b85f2e2")
!4 = !{!5, !8, !10, !11}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 32, align: 32)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!9 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!12 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!13 = !{}
!14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
!15 = distinct !DIGlobalVariable(name: "<&str as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !16, isLocal: true, isDefinition: true)
!16 = !DICompositeType(tag: DW_TAG_structure_type, name: "<&str as core::fmt::Debug>::{vtable_type}", file: !2, size: 128, align: 32, flags: DIFlagArtificial, elements: !17, vtableHolder: !22, templateParams: !13, identifier: "4c72bea2c43ff6a7f7303943d2ccbafe")
!17 = !{!18, !19, !20, !21}
!18 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !16, file: !2, baseType: !6, size: 32, align: 32)
!19 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !16, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!20 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !16, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!21 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !16, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!22 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 64, align: 32, elements: !23, templateParams: !13, identifier: "9277eecd40495f85161460476aacc992")
!23 = !{!24, !27}
!24 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !22, file: !2, baseType: !25, size: 32, align: 32)
!25 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!26 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!27 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !22, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
!29 = distinct !DIGlobalVariable(name: "<u8 as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !30, isLocal: true, isDefinition: true)
!30 = !DICompositeType(tag: DW_TAG_structure_type, name: "<u8 as core::fmt::Debug>::{vtable_type}", file: !2, size: 128, align: 32, flags: DIFlagArtificial, elements: !31, vtableHolder: !26, templateParams: !13, identifier: "c612a2affee42820c5ff622205e5f9de")
!31 = !{!32, !33, !34, !35}
!32 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !30, file: !2, baseType: !6, size: 32, align: 32)
!33 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !30, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!34 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !30, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!35 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !30, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
!37 = distinct !DIGlobalVariable(name: "<u32 as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !38, isLocal: true, isDefinition: true)
!38 = !DICompositeType(tag: DW_TAG_structure_type, name: "<u32 as core::fmt::Debug>::{vtable_type}", file: !2, size: 128, align: 32, flags: DIFlagArtificial, elements: !39, vtableHolder: !44, templateParams: !13, identifier: "19c70b5ae051baee956e0e118b17cd59")
!39 = !{!40, !41, !42, !43}
!40 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !38, file: !2, baseType: !6, size: 32, align: 32)
!41 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !38, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!42 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !38, file: !2, baseType: !9, size: 32, align: 32, offset: 64)
!43 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !38, file: !2, baseType: !6, size: 32, align: 32, offset: 96)
!44 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!45 = !{!"rustc version 1.92.0-nightly (6380899f3 2025-10-18)"}
!46 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !47, producer: "clang LLVM (rustc version 1.92.0-nightly (6380899f3 2025-10-18))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !48, splitDebugInlining: false, nameTableKind: None)
!47 = !DIFile(filename: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasi-0.11.1+wasi-snapshot-preview1/src/lib.rs/@/wasi.476d268f7a32d6a5-cgu.0", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasi-0.11.1+wasi-snapshot-preview1")
!48 = !{!0, !14, !28, !36}
!49 = !{i32 7, !"Dwarf Version", i32 4}
!50 = !{i32 2, !"Debug Info Version", i32 3}
!51 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h923b7f36013e4d7dE", scope: !53, file: !52, line: 85, type: !57, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !98)
!52 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "14f1acdd980d957a36bf4243cc704071")
!53 = !DINamespace(name: "{impl#58}", scope: !54)
!54 = !DINamespace(name: "num", scope: !55)
!55 = !DINamespace(name: "fmt", scope: !56)
!56 = !DINamespace(name: "core", scope: null)
!57 = !DISubroutineType(types: !58)
!58 = !{!59, !77, !78}
!59 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !60, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !61, templateParams: !13, identifier: "613ace46ae0c395d39c31f05d3934750")
!60 = !DINamespace(name: "result", scope: !56)
!61 = !{!62}
!62 = !DICompositeType(tag: DW_TAG_variant_part, scope: !59, file: !2, size: 8, align: 8, elements: !63, templateParams: !13, identifier: "2bd67c77928327a5a86e1d970227dbc3", discriminator: !76)
!63 = !{!64, !72}
!64 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !62, file: !2, baseType: !65, size: 8, align: 8, extraData: i8 0)
!65 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !59, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !66, templateParams: !68, identifier: "8e1fa5ea2cd8f77479a16f216aa53a42")
!66 = !{!67}
!67 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !65, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!68 = !{!69, !70}
!69 = !DITemplateTypeParameter(name: "T", type: !7)
!70 = !DITemplateTypeParameter(name: "E", type: !71)
!71 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !55, file: !2, align: 8, flags: DIFlagPublic, elements: !13, identifier: "cac4d2a6635a122844ffbe3b52a15933")
!72 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !62, file: !2, baseType: !73, size: 8, align: 8, extraData: i8 1)
!73 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !59, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !74, templateParams: !68, identifier: "bd8eb8fbb58ca24e2467a7f35c864471")
!74 = !{!75}
!75 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !73, file: !2, baseType: !71, align: 8, offset: 8, flags: DIFlagPublic)
!76 = !DIDerivedType(tag: DW_TAG_member, scope: !59, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagArtificial)
!77 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u8", baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!78 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !79, size: 32, align: 32, dwarfAddressSpace: 0)
!79 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !55, file: !2, size: 128, align: 32, flags: DIFlagPublic, elements: !80, templateParams: !13, identifier: "9c19c8ef0b5ae3cad350e741e841742c")
!80 = !{!81, !87}
!81 = !DIDerivedType(tag: DW_TAG_member, name: "options", scope: !79, file: !2, baseType: !82, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!82 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormattingOptions", scope: !55, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !83, templateParams: !13, identifier: "8e7d20540a73fe2190308d0618721e3e")
!83 = !{!84, !85, !86}
!84 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !82, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagPrivate)
!85 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !82, file: !2, baseType: !12, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!86 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !82, file: !2, baseType: !12, size: 16, align: 16, offset: 48, flags: DIFlagPrivate)
!87 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !79, file: !2, baseType: !88, size: 64, align: 32, flags: DIFlagPrivate)
!88 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 64, align: 32, elements: !89, templateParams: !13, identifier: "ed1fc41b72305de4afb5dbb44887680d")
!89 = !{!90, !93}
!90 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !88, file: !2, baseType: !91, size: 32, align: 32)
!91 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !92, size: 32, align: 32, dwarfAddressSpace: 0)
!92 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !13, identifier: "3bd7022d6bc7a1bba9386a42dfa7db9d")
!93 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !88, file: !2, baseType: !94, size: 32, align: 32, offset: 32)
!94 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !95, size: 32, align: 32, dwarfAddressSpace: 0)
!95 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 192, align: 32, elements: !96)
!96 = !{!97}
!97 = !DISubrange(count: 6, lowerBound: 0)
!98 = !{!99, !100}
!99 = !DILocalVariable(name: "self", arg: 1, scope: !51, file: !52, line: 85, type: !77)
!100 = !DILocalVariable(name: "f", arg: 2, scope: !51, file: !52, line: 85, type: !78)
!101 = !DILocation(line: 85, column: 24, scope: !51)
!102 = !DILocation(line: 85, column: 31, scope: !51)
!103 = !DILocation(line: 86, column: 26, scope: !51)
!104 = !DILocation(line: 86, column: 24, scope: !51)
!105 = !DILocation(line: 88, column: 33, scope: !51)
!106 = !DILocation(line: 88, column: 31, scope: !51)
!107 = !DILocation(line: 87, column: 25, scope: !51)
!108 = !DILocation(line: 91, column: 25, scope: !51)
!109 = !DILocation(line: 89, column: 25, scope: !51)
!110 = !DILocation(line: 93, column: 18, scope: !51)
!111 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u16$GT$3fmt17hc63df5a2f7a3da02E", scope: !112, file: !52, line: 85, type: !113, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !116)
!112 = !DINamespace(name: "{impl#59}", scope: !54)
!113 = !DISubroutineType(types: !114)
!114 = !{!59, !115, !78}
!115 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u16", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!116 = !{!117, !118}
!117 = !DILocalVariable(name: "self", arg: 1, scope: !111, file: !52, line: 85, type: !115)
!118 = !DILocalVariable(name: "f", arg: 2, scope: !111, file: !52, line: 85, type: !78)
!119 = !DILocation(line: 85, column: 24, scope: !111)
!120 = !DILocation(line: 85, column: 31, scope: !111)
!121 = !DILocation(line: 86, column: 26, scope: !111)
!122 = !DILocation(line: 86, column: 24, scope: !111)
!123 = !DILocation(line: 88, column: 33, scope: !111)
!124 = !DILocation(line: 88, column: 31, scope: !111)
!125 = !DILocation(line: 87, column: 25, scope: !111)
!126 = !DILocation(line: 91, column: 25, scope: !111)
!127 = !DILocation(line: 89, column: 25, scope: !111)
!128 = !DILocation(line: 93, column: 18, scope: !111)
!129 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17hfbd5c8efd222f376E", scope: !130, file: !52, line: 85, type: !131, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !134)
!130 = !DINamespace(name: "{impl#60}", scope: !54)
!131 = !DISubroutineType(types: !132)
!132 = !{!59, !133, !78}
!133 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u32", baseType: !44, size: 32, align: 32, dwarfAddressSpace: 0)
!134 = !{!135, !136}
!135 = !DILocalVariable(name: "self", arg: 1, scope: !129, file: !52, line: 85, type: !133)
!136 = !DILocalVariable(name: "f", arg: 2, scope: !129, file: !52, line: 85, type: !78)
!137 = !DILocation(line: 85, column: 24, scope: !129)
!138 = !DILocation(line: 85, column: 31, scope: !129)
!139 = !DILocation(line: 86, column: 26, scope: !129)
!140 = !DILocation(line: 86, column: 24, scope: !129)
!141 = !DILocation(line: 88, column: 33, scope: !129)
!142 = !DILocation(line: 88, column: 31, scope: !129)
!143 = !DILocation(line: 87, column: 25, scope: !129)
!144 = !DILocation(line: 91, column: 25, scope: !129)
!145 = !DILocation(line: 89, column: 25, scope: !129)
!146 = !DILocation(line: 93, column: 18, scope: !129)
!147 = distinct !DISubprogram(name: "as_statically_known_str", linkageName: "_ZN4core3fmt9Arguments23as_statically_known_str17h4ba4e91277018bdbE", scope: !149, file: !148, line: 717, type: !237, scopeLine: 717, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !253, retainedNodes: !254)
!148 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "953353698add15f5879168b0ac566843")
!149 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !55, file: !2, size: 192, align: 32, flags: DIFlagPublic, elements: !150, templateParams: !13, identifier: "d691e62b2ee4847c2af32873f04bd10")
!150 = !{!151, !157, !199}
!151 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !149, file: !2, baseType: !152, size: 64, align: 32, flags: DIFlagPrivate)
!152 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 64, align: 32, elements: !153, templateParams: !13, identifier: "4e66b00a376d6af5b8765440fb2839f")
!153 = !{!154, !156}
!154 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !152, file: !2, baseType: !155, size: 32, align: 32)
!155 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !22, size: 32, align: 32, dwarfAddressSpace: 0)
!156 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !152, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!157 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !149, file: !2, baseType: !158, size: 64, align: 32, offset: 128, flags: DIFlagPrivate)
!158 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !159, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !160, templateParams: !13, identifier: "a638667a460b22fe10961f9a2f3202aa")
!159 = !DINamespace(name: "option", scope: !56)
!160 = !{!161}
!161 = !DICompositeType(tag: DW_TAG_variant_part, scope: !158, file: !2, size: 64, align: 32, elements: !162, templateParams: !13, identifier: "29af53ccc7f21f4d5671e352d673889a", discriminator: !198)
!162 = !{!163, !194}
!163 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !161, file: !2, baseType: !164, size: 64, align: 32, extraData: i32 0)
!164 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !158, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, templateParams: !165, identifier: "11ce4f4d10f67887bbe6bf59a521c479")
!165 = !{!166}
!166 = !DITemplateTypeParameter(name: "T", type: !167)
!167 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 64, align: 32, elements: !168, templateParams: !13, identifier: "b0485535d7020130e949c24f3fc2aa00")
!168 = !{!169, !193}
!169 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !167, file: !2, baseType: !170, size: 32, align: 32)
!170 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !171, size: 32, align: 32, dwarfAddressSpace: 0)
!171 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !172, file: !2, size: 192, align: 32, flags: DIFlagPublic, elements: !173, templateParams: !13, identifier: "8cb06f9d78dc629c8f52fc3b5544996c")
!172 = !DINamespace(name: "rt", scope: !55)
!173 = !{!174, !175, !176, !192}
!174 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !171, file: !2, baseType: !9, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
!175 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !171, file: !2, baseType: !44, size: 32, align: 32, offset: 160, flags: DIFlagPublic)
!176 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !171, file: !2, baseType: !177, size: 64, align: 32, flags: DIFlagPublic)
!177 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !172, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !178, templateParams: !13, identifier: "2d7772037f5c744e87d41105441784d5")
!178 = !{!179}
!179 = !DICompositeType(tag: DW_TAG_variant_part, scope: !177, file: !2, size: 64, align: 32, elements: !180, templateParams: !13, identifier: "af14687975a61e1ae6bbcdaeb79a8a2", discriminator: !191)
!180 = !{!181, !185, !189}
!181 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !179, file: !2, baseType: !182, size: 64, align: 32, extraData: i16 0)
!182 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !177, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !183, templateParams: !13, identifier: "da16c9b5356522ffb015c0e99237342e")
!183 = !{!184}
!184 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !182, file: !2, baseType: !12, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!185 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !179, file: !2, baseType: !186, size: 64, align: 32, extraData: i16 1)
!186 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !177, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !187, templateParams: !13, identifier: "8d84b26eccf0f48fe70ea50c79b83fc9")
!187 = !{!188}
!188 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !186, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!189 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !179, file: !2, baseType: !190, size: 64, align: 32, extraData: i16 2)
!190 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !177, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, identifier: "e4d910bcc0c2da0048af65cce9b02bdf")
!191 = !DIDerivedType(tag: DW_TAG_member, scope: !177, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!192 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !171, file: !2, baseType: !177, size: 64, align: 32, offset: 64, flags: DIFlagPublic)
!193 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !167, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!194 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !161, file: !2, baseType: !195, size: 64, align: 32)
!195 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !158, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !196, templateParams: !165, identifier: "b6f59188292a44db7736125146b92cb0")
!196 = !{!197}
!197 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !195, file: !2, baseType: !167, size: 64, align: 32, flags: DIFlagPublic)
!198 = !DIDerivedType(tag: DW_TAG_member, scope: !158, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagArtificial)
!199 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !149, file: !2, baseType: !200, size: 64, align: 32, offset: 64, flags: DIFlagPrivate)
!200 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 64, align: 32, elements: !201, templateParams: !13, identifier: "14634098cacc86d372c43019bc81f26f")
!201 = !{!202, !236}
!202 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !200, file: !2, baseType: !203, size: 32, align: 32)
!203 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !204, size: 32, align: 32, dwarfAddressSpace: 0)
!204 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !172, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !205, templateParams: !13, identifier: "14dca3c1b1040cd8e8db0eaa112c8216")
!205 = !{!206}
!206 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !204, file: !2, baseType: !207, size: 64, align: 32, flags: DIFlagPrivate)
!207 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !172, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !208, templateParams: !13, identifier: "fb1492950c21086074bab206592842dc")
!208 = !{!209}
!209 = !DICompositeType(tag: DW_TAG_variant_part, scope: !207, file: !2, size: 64, align: 32, elements: !210, templateParams: !13, identifier: "478e018ae6e38e2110d0d424641ab18", discriminator: !235)
!210 = !{!211, !231}
!211 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !209, file: !2, baseType: !212, size: 64, align: 32)
!212 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !207, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !213, templateParams: !13, identifier: "59bc7f5c5a99ab4be3c3f06b9190c327")
!213 = !{!214, !221, !225}
!214 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !212, file: !2, baseType: !215, size: 32, align: 32, flags: DIFlagPrivate)
!215 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<()>", scope: !216, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !218, templateParams: !220, identifier: "d9f2bcb64deb934daba9b509aea4a83e")
!216 = !DINamespace(name: "non_null", scope: !217)
!217 = !DINamespace(name: "ptr", scope: !56)
!218 = !{!219}
!219 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !215, file: !2, baseType: !6, size: 32, align: 32, flags: DIFlagPrivate)
!220 = !{!69}
!221 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !212, file: !2, baseType: !222, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!222 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "unsafe fn(core::ptr::non_null::NonNull<()>, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !223, size: 32, align: 32, dwarfAddressSpace: 0)
!223 = !DISubroutineType(types: !224)
!224 = !{!59, !215, !78}
!225 = !DIDerivedType(tag: DW_TAG_member, name: "_lifetime", scope: !212, file: !2, baseType: !226, align: 8, offset: 64, flags: DIFlagPrivate)
!226 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&()>", scope: !227, file: !2, align: 8, flags: DIFlagPublic, elements: !13, templateParams: !228, identifier: "e71ee38df7dbfccdae82d3411c10d5bc")
!227 = !DINamespace(name: "marker", scope: !56)
!228 = !{!229}
!229 = !DITemplateTypeParameter(name: "T", type: !230)
!230 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !7, size: 32, align: 32, dwarfAddressSpace: 0)
!231 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !209, file: !2, baseType: !232, size: 64, align: 32, extraData: i32 0)
!232 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !207, file: !2, size: 64, align: 32, flags: DIFlagPrivate, elements: !233, templateParams: !13, identifier: "bcc61db69ea5777ac138ac099ea396b2")
!233 = !{!234}
!234 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !232, file: !2, baseType: !12, size: 16, align: 16, offset: 32, flags: DIFlagPrivate)
!235 = !DIDerivedType(tag: DW_TAG_member, scope: !207, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagArtificial)
!236 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !200, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!237 = !DISubroutineType(types: !238)
!238 = !{!239, !252}
!239 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&str>", scope: !159, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !240, templateParams: !13, identifier: "70526b74386e3ab1af24a4552995aad0")
!240 = !{!241}
!241 = !DICompositeType(tag: DW_TAG_variant_part, scope: !239, file: !2, size: 64, align: 32, elements: !242, templateParams: !13, identifier: "8075e3d3cbf81a82fddc7ee972736375", discriminator: !251)
!242 = !{!243, !247}
!243 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !241, file: !2, baseType: !244, size: 64, align: 32, extraData: i32 0)
!244 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !239, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !13, templateParams: !245, identifier: "a2c8c52cbf664b15e04ba33a9d1fb455")
!245 = !{!246}
!246 = !DITemplateTypeParameter(name: "T", type: !22)
!247 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !241, file: !2, baseType: !248, size: 64, align: 32)
!248 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !239, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !249, templateParams: !245, identifier: "b664394454dbb74539919442d1cb2e90")
!249 = !{!250}
!250 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !248, file: !2, baseType: !22, size: 64, align: 32, flags: DIFlagPublic)
!251 = !DIDerivedType(tag: DW_TAG_member, scope: !239, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagArtificial)
!252 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Arguments", baseType: !149, size: 32, align: 32, dwarfAddressSpace: 0)
!253 = !DISubprogram(name: "as_statically_known_str", linkageName: "_ZN4core3fmt9Arguments23as_statically_known_str17h4ba4e91277018bdbE", scope: !149, file: !148, line: 717, type: !237, scopeLine: 717, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!254 = !{!255, !256}
!255 = !DILocalVariable(name: "self", arg: 1, scope: !147, file: !148, line: 717, type: !252)
!256 = !DILocalVariable(name: "s", scope: !257, file: !148, line: 718, type: !239, align: 32)
!257 = distinct !DILexicalBlock(scope: !147, file: !148, line: 718, column: 9)
!258 = !DILocation(line: 717, column: 36, scope: !147)
!259 = !DILocation(line: 718, column: 13, scope: !257)
!260 = !DILocation(line: 718, column: 22, scope: !147)
!261 = !DILocation(line: 719, column: 56, scope: !257)
!262 = !DILocation(line: 719, column: 12, scope: !257)
!263 = !DILocation(line: 719, column: 80, scope: !257)
!264 = !DILocation(line: 719, column: 9, scope: !257)
!265 = !DILocation(line: 719, column: 69, scope: !257)
!266 = !DILocation(line: 720, column: 6, scope: !147)
!267 = distinct !DISubprogram(name: "as_str", linkageName: "_ZN4core3fmt9Arguments6as_str17h5f43f546cd1c7996E", scope: !149, file: !148, line: 704, type: !237, scopeLine: 704, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !268, retainedNodes: !269)
!268 = !DISubprogram(name: "as_str", linkageName: "_ZN4core3fmt9Arguments6as_str17h5f43f546cd1c7996E", scope: !149, file: !148, line: 704, type: !237, scopeLine: 704, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!269 = !{!270, !271}
!270 = !DILocalVariable(name: "self", arg: 1, scope: !267, file: !148, line: 704, type: !252)
!271 = !DILocalVariable(name: "s", scope: !272, file: !148, line: 707, type: !273, align: 32)
!272 = distinct !DILexicalBlock(scope: !267, file: !148, line: 707, column: 13)
!273 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&str", baseType: !22, size: 32, align: 32, dwarfAddressSpace: 0)
!274 = !DILocation(line: 704, column: 25, scope: !267)
!275 = !DILocation(line: 705, column: 16, scope: !267)
!276 = !DILocation(line: 705, column: 29, scope: !267)
!277 = !DILocation(line: 705, column: 15, scope: !267)
!278 = !DILocation(line: 706, column: 14, scope: !267)
!279 = !DILocation(line: 706, column: 18, scope: !267)
!280 = !DILocation(line: 707, column: 14, scope: !267)
!281 = !DILocation(line: 706, column: 25, scope: !267)
!282 = !DILocation(line: 706, column: 32, scope: !267)
!283 = !DILocation(line: 708, column: 18, scope: !267)
!284 = !DILocation(line: 710, column: 6, scope: !267)
!285 = !DILocation(line: 707, column: 19, scope: !267)
!286 = !DILocation(line: 707, column: 15, scope: !267)
!287 = !DILocation(line: 707, column: 15, scope: !272)
!288 = !DILocation(line: 707, column: 31, scope: !272)
!289 = !DILocation(line: 707, column: 26, scope: !272)
!290 = !DILocation(line: 707, column: 32, scope: !267)
!291 = distinct !DISubprogram(name: "write_fmt", linkageName: "_ZN4core3fmt9Formatter9write_fmt17h5e1a4779fbbec593E", scope: !79, file: !148, line: 1915, type: !292, scopeLine: 1915, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !294, retainedNodes: !295)
!292 = !DISubroutineType(types: !293)
!293 = !{!59, !78, !149}
!294 = !DISubprogram(name: "write_fmt", linkageName: "_ZN4core3fmt9Formatter9write_fmt17h5e1a4779fbbec593E", scope: !79, file: !148, line: 1915, type: !292, scopeLine: 1915, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !13)
!295 = !{!296, !297, !298}
!296 = !DILocalVariable(name: "self", arg: 1, scope: !291, file: !148, line: 1915, type: !78)
!297 = !DILocalVariable(name: "fmt", arg: 2, scope: !291, file: !148, line: 1915, type: !149)
!298 = !DILocalVariable(name: "s", scope: !299, file: !148, line: 1916, type: !22, align: 32)
!299 = distinct !DILexicalBlock(scope: !291, file: !148, line: 1916, column: 56)
!300 = !DILocation(line: 1915, column: 22, scope: !291)
!301 = !DILocation(line: 1915, column: 33, scope: !291)
!302 = !DILocation(line: 1916, column: 30, scope: !299)
!303 = !DILocation(line: 1916, column: 26, scope: !299)
!304 = !DILocation(line: 1916, column: 16, scope: !299)
!305 = !DILocation(line: 1916, column: 21, scope: !299)
!306 = !DILocation(line: 1917, column: 13, scope: !299)
!307 = !DILocation(line: 1917, column: 22, scope: !299)
!308 = !DILocation(line: 1919, column: 19, scope: !291)
!309 = !DILocation(line: 1919, column: 13, scope: !291)
!310 = !DILocation(line: 1921, column: 6, scope: !291)
!311 = !DILocation(line: 1915, column: 5, scope: !291)
!312 = distinct !DISubprogram(name: "read<u16>", linkageName: "_ZN4core3ptr4read17h188bb76cc8094201E", scope: !217, file: !313, line: 1705, type: !314, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !338, retainedNodes: !336)
!313 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "8857c34524728cc5887872677b8e1917")
!314 = !DISubroutineType(types: !315)
!315 = !{!12, !316, !317}
!316 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u16", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!317 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !318, size: 32, align: 32, dwarfAddressSpace: 0)
!318 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !319, file: !2, size: 128, align: 32, flags: DIFlagPublic, elements: !321, templateParams: !13, identifier: "7c34cafe8ea1dcad4032b9360816105f")
!319 = !DINamespace(name: "location", scope: !320)
!320 = !DINamespace(name: "panic", scope: !56)
!321 = !{!322, !332, !333, !334}
!322 = !DIDerivedType(tag: DW_TAG_member, name: "filename", scope: !318, file: !2, baseType: !323, size: 64, align: 32, flags: DIFlagPrivate)
!323 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<str>", scope: !216, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !324, templateParams: !330, identifier: "88212fc410c4399fd5095990cc8304ca")
!324 = !{!325}
!325 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !323, file: !2, baseType: !326, size: 64, align: 32, flags: DIFlagPrivate)
!326 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const str", file: !2, size: 64, align: 32, elements: !327, templateParams: !13, identifier: "238a44609877474087c05adf26cd41fa")
!327 = !{!328, !329}
!328 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !326, file: !2, baseType: !25, size: 32, align: 32)
!329 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !326, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!330 = !{!331}
!331 = !DITemplateTypeParameter(name: "T", type: !26)
!332 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !318, file: !2, baseType: !44, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!333 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !318, file: !2, baseType: !44, size: 32, align: 32, offset: 96, flags: DIFlagPrivate)
!334 = !DIDerivedType(tag: DW_TAG_member, name: "_filename", scope: !318, file: !2, baseType: !335, align: 8, offset: 128, flags: DIFlagPrivate)
!335 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&str>", scope: !227, file: !2, align: 8, flags: DIFlagPublic, elements: !13, templateParams: !245, identifier: "4cfc3eea77dd95eabd59051b67bd7e66")
!336 = !{!337}
!337 = !DILocalVariable(name: "src", arg: 1, scope: !312, file: !313, line: 1705, type: !316)
!338 = !{!339}
!339 = !DITemplateTypeParameter(name: "T", type: !12)
!340 = !DILocation(line: 1705, column: 29, scope: !312)
!341 = !DILocation(line: 77, column: 35, scope: !342)
!342 = !DILexicalBlockFile(scope: !312, file: !343, discriminator: 0)
!343 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ub_checks.rs", directory: "", checksumkind: CSK_MD5, checksum: "41b3943b2b7dc8c218ee37ead81b317d")
!344 = !DILocation(line: 1744, column: 9, scope: !312)
!345 = !DILocation(line: 1746, column: 2, scope: !312)
!346 = !DILocation(line: 78, column: 17, scope: !342)
!347 = distinct !DISubprogram(name: "read<wasi::lib_generated::Fdstat>", linkageName: "_ZN4core3ptr4read17h28b571f9788304ccE", scope: !217, file: !313, line: 1705, type: !348, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !365, retainedNodes: !363)
!348 = !DISubroutineType(types: !349)
!349 = !{!350, !362, !317}
!350 = !DICompositeType(tag: DW_TAG_structure_type, name: "Fdstat", scope: !351, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !353, templateParams: !13, identifier: "e494ba5722f7ff0de34bf1d7151eb2f3")
!351 = !DINamespace(name: "lib_generated", scope: !352)
!352 = !DINamespace(name: "wasi", scope: null)
!353 = !{!354, !358, !359, !361}
!354 = !DIDerivedType(tag: DW_TAG_member, name: "fs_filetype", scope: !350, file: !2, baseType: !355, size: 8, align: 8, flags: DIFlagPublic)
!355 = !DICompositeType(tag: DW_TAG_structure_type, name: "Filetype", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !356, templateParams: !13, identifier: "24877855dc10e77c2f78d2abc8a35243")
!356 = !{!357}
!357 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !355, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!358 = !DIDerivedType(tag: DW_TAG_member, name: "fs_flags", scope: !350, file: !2, baseType: !12, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!359 = !DIDerivedType(tag: DW_TAG_member, name: "fs_rights_base", scope: !350, file: !2, baseType: !360, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!360 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!361 = !DIDerivedType(tag: DW_TAG_member, name: "fs_rights_inheriting", scope: !350, file: !2, baseType: !360, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
!362 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Fdstat", baseType: !350, size: 32, align: 32, dwarfAddressSpace: 0)
!363 = !{!364}
!364 = !DILocalVariable(name: "src", arg: 1, scope: !347, file: !313, line: 1705, type: !362)
!365 = !{!366}
!366 = !DITemplateTypeParameter(name: "T", type: !350)
!367 = !DILocation(line: 1705, column: 29, scope: !347)
!368 = !DILocation(line: 77, column: 35, scope: !369)
!369 = !DILexicalBlockFile(scope: !347, file: !343, discriminator: 0)
!370 = !DILocation(line: 1744, column: 9, scope: !347)
!371 = !DILocation(line: 1746, column: 2, scope: !347)
!372 = !DILocation(line: 78, column: 17, scope: !369)
!373 = distinct !DISubprogram(name: "read<u64>", linkageName: "_ZN4core3ptr4read17h487dc6145fad69b1E", scope: !217, file: !313, line: 1705, type: !374, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !379, retainedNodes: !377)
!374 = !DISubroutineType(types: !375)
!375 = !{!360, !376, !317}
!376 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u64", baseType: !360, size: 32, align: 32, dwarfAddressSpace: 0)
!377 = !{!378}
!378 = !DILocalVariable(name: "src", arg: 1, scope: !373, file: !313, line: 1705, type: !376)
!379 = !{!380}
!380 = !DITemplateTypeParameter(name: "T", type: !360)
!381 = !DILocation(line: 1705, column: 29, scope: !373)
!382 = !DILocation(line: 77, column: 35, scope: !383)
!383 = !DILexicalBlockFile(scope: !373, file: !343, discriminator: 0)
!384 = !DILocation(line: 1744, column: 9, scope: !373)
!385 = !DILocation(line: 1746, column: 2, scope: !373)
!386 = !DILocation(line: 78, column: 17, scope: !383)
!387 = distinct !DISubprogram(name: "read<wasi::lib_generated::Prestat>", linkageName: "_ZN4core3ptr4read17h58671998979cdf8bE", scope: !217, file: !313, line: 1705, type: !388, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !403, retainedNodes: !401)
!388 = !DISubroutineType(types: !389)
!389 = !{!390, !400, !317}
!390 = !DICompositeType(tag: DW_TAG_structure_type, name: "Prestat", scope: !351, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !391, templateParams: !13, identifier: "1a06d4c03dc019c192407bcb1cd592e")
!391 = !{!392, !393}
!392 = !DIDerivedType(tag: DW_TAG_member, name: "tag", scope: !390, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPublic)
!393 = !DIDerivedType(tag: DW_TAG_member, name: "u", scope: !390, file: !2, baseType: !394, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!394 = !DICompositeType(tag: DW_TAG_union_type, name: "PrestatU", scope: !351, file: !2, size: 32, align: 32, elements: !395, templateParams: !13, identifier: "17f75b084371c9103c355dc9bf61522")
!395 = !{!396}
!396 = !DIDerivedType(tag: DW_TAG_member, name: "dir", scope: !394, file: !2, baseType: !397, size: 32, align: 32)
!397 = !DICompositeType(tag: DW_TAG_structure_type, name: "PrestatDir", scope: !351, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !398, templateParams: !13, identifier: "395615a0667192f93013bba3e153006")
!398 = !{!399}
!399 = !DIDerivedType(tag: DW_TAG_member, name: "pr_name_len", scope: !397, file: !2, baseType: !9, size: 32, align: 32, flags: DIFlagPublic)
!400 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Prestat", baseType: !390, size: 32, align: 32, dwarfAddressSpace: 0)
!401 = !{!402}
!402 = !DILocalVariable(name: "src", arg: 1, scope: !387, file: !313, line: 1705, type: !400)
!403 = !{!404}
!404 = !DITemplateTypeParameter(name: "T", type: !390)
!405 = !DILocation(line: 1705, column: 29, scope: !387)
!406 = !DILocation(line: 77, column: 35, scope: !407)
!407 = !DILexicalBlockFile(scope: !387, file: !343, discriminator: 0)
!408 = !DILocation(line: 1744, column: 9, scope: !387)
!409 = !DILocation(line: 1746, column: 2, scope: !387)
!410 = !DILocation(line: 78, column: 17, scope: !407)
!411 = distinct !DISubprogram(name: "read<wasi::lib_generated::Filestat>", linkageName: "_ZN4core3ptr4read17hc08313af9d479144E", scope: !217, file: !313, line: 1705, type: !412, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !427, retainedNodes: !425)
!412 = !DISubroutineType(types: !413)
!413 = !{!414, !424, !317}
!414 = !DICompositeType(tag: DW_TAG_structure_type, name: "Filestat", scope: !351, file: !2, size: 512, align: 64, flags: DIFlagPublic, elements: !415, templateParams: !13, identifier: "ecc2d135f2b156ff837ac30d7fe012a1")
!415 = !{!416, !417, !418, !419, !420, !421, !422, !423}
!416 = !DIDerivedType(tag: DW_TAG_member, name: "dev", scope: !414, file: !2, baseType: !360, size: 64, align: 64, flags: DIFlagPublic)
!417 = !DIDerivedType(tag: DW_TAG_member, name: "ino", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!418 = !DIDerivedType(tag: DW_TAG_member, name: "filetype", scope: !414, file: !2, baseType: !355, size: 8, align: 8, offset: 128, flags: DIFlagPublic)
!419 = !DIDerivedType(tag: DW_TAG_member, name: "nlink", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 192, flags: DIFlagPublic)
!420 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!421 = !DIDerivedType(tag: DW_TAG_member, name: "atim", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 320, flags: DIFlagPublic)
!422 = !DIDerivedType(tag: DW_TAG_member, name: "mtim", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 384, flags: DIFlagPublic)
!423 = !DIDerivedType(tag: DW_TAG_member, name: "ctim", scope: !414, file: !2, baseType: !360, size: 64, align: 64, offset: 448, flags: DIFlagPublic)
!424 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Filestat", baseType: !414, size: 32, align: 32, dwarfAddressSpace: 0)
!425 = !{!426}
!426 = !DILocalVariable(name: "src", arg: 1, scope: !411, file: !313, line: 1705, type: !424)
!427 = !{!428}
!428 = !DITemplateTypeParameter(name: "T", type: !414)
!429 = !DILocation(line: 1705, column: 29, scope: !411)
!430 = !DILocation(line: 77, column: 35, scope: !431)
!431 = !DILexicalBlockFile(scope: !411, file: !343, discriminator: 0)
!432 = !DILocation(line: 1744, column: 9, scope: !411)
!433 = !DILocation(line: 1746, column: 2, scope: !411)
!434 = !DILocation(line: 78, column: 17, scope: !431)
!435 = distinct !DISubprogram(name: "read<u32>", linkageName: "_ZN4core3ptr4read17he4d71e30ba8af448E", scope: !217, file: !313, line: 1705, type: !436, scopeLine: 1705, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !441, retainedNodes: !439)
!436 = !DISubroutineType(types: !437)
!437 = !{!44, !438, !317}
!438 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u32", baseType: !44, size: 32, align: 32, dwarfAddressSpace: 0)
!439 = !{!440}
!440 = !DILocalVariable(name: "src", arg: 1, scope: !435, file: !313, line: 1705, type: !438)
!441 = !{!442}
!442 = !DITemplateTypeParameter(name: "T", type: !44)
!443 = !DILocation(line: 1705, column: 29, scope: !435)
!444 = !DILocation(line: 77, column: 35, scope: !445)
!445 = !DILexicalBlockFile(scope: !435, file: !343, discriminator: 0)
!446 = !DILocation(line: 1744, column: 9, scope: !435)
!447 = !DILocation(line: 1746, column: 2, scope: !435)
!448 = !DILocation(line: 78, column: 17, scope: !445)
!449 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core3ptr4read18precondition_check17h1f0d0df461b2cb3fE", scope: !450, file: !343, line: 68, type: !451, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !454)
!450 = !DINamespace(name: "read", scope: !217)
!451 = !DISubroutineType(types: !452)
!452 = !{null, !6, !9, !453, !317}
!453 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!454 = !{!455, !456, !457, !458}
!455 = !DILocalVariable(name: "addr", arg: 1, scope: !449, file: !343, line: 68, type: !6)
!456 = !DILocalVariable(name: "align", arg: 2, scope: !449, file: !343, line: 68, type: !9)
!457 = !DILocalVariable(name: "is_zst", arg: 3, scope: !449, file: !343, line: 68, type: !453)
!458 = !DILocalVariable(name: "msg", scope: !459, file: !343, line: 70, type: !22, align: 32)
!459 = distinct !DILexicalBlock(scope: !449, file: !343, line: 70, column: 21)
!460 = !DILocation(line: 68, column: 43, scope: !449)
!461 = !DILocation(line: 70, column: 25, scope: !459)
!462 = !DILocation(line: 1742, column: 18, scope: !463)
!463 = !DILexicalBlockFile(scope: !449, file: !313, discriminator: 0)
!464 = !DILocation(line: 73, column: 94, scope: !459)
!465 = !DILocation(line: 73, column: 59, scope: !459)
!466 = !DILocation(line: 73, column: 21, scope: !459)
!467 = !DILocation(line: 75, column: 14, scope: !449)
!468 = distinct !DISubprogram(name: "len", linkageName: "_ZN4core3str21_$LT$impl$u20$str$GT$3len17h3af2aaf307a9ee60E", scope: !470, file: !469, line: 141, type: !472, scopeLine: 141, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !474)
!469 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "361734e74e585b99fb3835c9168d18d7")
!470 = !DINamespace(name: "{impl#0}", scope: !471)
!471 = !DINamespace(name: "str", scope: !56)
!472 = !DISubroutineType(types: !473)
!473 = !{!9, !22}
!474 = !{!475}
!475 = !DILocalVariable(name: "self", arg: 1, scope: !468, file: !469, line: 141, type: !22)
!476 = !DILocation(line: 141, column: 22, scope: !468)
!477 = !DILocalVariable(name: "self", arg: 1, scope: !478, file: !469, line: 486, type: !22)
!478 = distinct !DISubprogram(name: "as_bytes", linkageName: "_ZN4core3str21_$LT$impl$u20$str$GT$8as_bytes17h9707b0eb27d72843E", scope: !470, file: !469, line: 486, type: !479, scopeLine: 486, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !485)
!479 = !DISubroutineType(types: !480)
!480 = !{!481, !22}
!481 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[u8]", file: !2, size: 64, align: 32, elements: !482, templateParams: !13, identifier: "31681e0c10b314f1f33e38b2779acbb4")
!482 = !{!483, !484}
!483 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !481, file: !2, baseType: !25, size: 32, align: 32)
!484 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !481, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!485 = !{!477}
!486 = !DILocation(line: 486, column: 27, scope: !478, inlinedAt: !487)
!487 = distinct !DILocation(line: 142, column: 14, scope: !468)
!488 = !DILocation(line: 489, column: 6, scope: !478, inlinedAt: !487)
!489 = !DILocation(line: 142, column: 14, scope: !468)
!490 = !DILocation(line: 143, column: 6, scope: !468)
!491 = distinct !DISubprogram(name: "unreachable_unchecked", linkageName: "_ZN4core4hint21unreachable_unchecked17h9b3c9dbd75c21290E", scope: !493, file: !492, line: 102, type: !494, scopeLine: 102, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13)
!492 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "56f659f9cbc57d60ad8ce456b7f06ccb")
!493 = !DINamespace(name: "hint", scope: !56)
!494 = !DISubroutineType(types: !495)
!495 = !{null, !317}
!496 = !DILocation(line: 77, column: 35, scope: !497)
!497 = !DILexicalBlockFile(scope: !491, file: !343, discriminator: 0)
!498 = !DILocation(line: 110, column: 14, scope: !491)
!499 = !DILocation(line: 78, column: 17, scope: !497)
!500 = distinct !DISubprogram(name: "precondition_check", linkageName: "_ZN4core4hint21unreachable_unchecked18precondition_check17ha0410ca2683f1fabE", scope: !501, file: !343, line: 68, type: !494, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !502)
!501 = !DINamespace(name: "unreachable_unchecked", scope: !493)
!502 = !{!503}
!503 = !DILocalVariable(name: "msg", scope: !504, file: !343, line: 70, type: !22, align: 32)
!504 = distinct !DILexicalBlock(scope: !500, file: !343, line: 70, column: 21)
!505 = !DILocation(line: 70, column: 25, scope: !504)
!506 = !DILocation(line: 73, column: 94, scope: !504)
!507 = !DILocation(line: 73, column: 59, scope: !504)
!508 = !DILocation(line: 73, column: 21, scope: !504)
!509 = distinct !DISubprogram(name: "panic_nounwind_fmt", linkageName: "_ZN4core9panicking18panic_nounwind_fmt17h898da05acec7acadE", scope: !511, file: !510, line: 95, type: !512, scopeLine: 95, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !514)
!510 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs", directory: "", checksumkind: CSK_MD5, checksum: "b120da646d1a09f31201b8a519374e57")
!511 = !DINamespace(name: "panicking", scope: !56)
!512 = !DISubroutineType(types: !513)
!513 = !{null, !149, !453, !317}
!514 = !{!515, !516}
!515 = !DILocalVariable(name: "fmt", arg: 1, scope: !509, file: !510, line: 95, type: !149)
!516 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !509, file: !510, line: 95, type: !453)
!517 = !DILocation(line: 95, column: 33, scope: !509)
!518 = !DILocation(line: 95, column: 58, scope: !509)
!519 = !DILocation(line: 2435, column: 27, scope: !520)
!520 = !DILexicalBlockFile(scope: !509, file: !521, discriminator: 0)
!521 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "5088527a679dbab229c7a43df7f388f7")
!522 = !DILocation(line: 2435, column: 9, scope: !520)
!523 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9panicking18panic_nounwind_fmt7runtime17h0317e615256657b0E", scope: !524, file: !521, line: 2423, type: !512, scopeLine: 2423, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !525)
!524 = !DINamespace(name: "panic_nounwind_fmt", scope: !511)
!525 = !{!526, !527, !528}
!526 = !DILocalVariable(name: "fmt", arg: 1, scope: !523, file: !521, line: 2423, type: !149)
!527 = !DILocalVariable(name: "force_no_backtrace", arg: 2, scope: !523, file: !521, line: 2423, type: !453)
!528 = !DILocalVariable(name: "pi", scope: !529, file: !510, line: 114, type: !530, align: 32)
!529 = distinct !DILexicalBlock(scope: !523, file: !510, line: 114, column: 13)
!530 = !DICompositeType(tag: DW_TAG_structure_type, name: "PanicInfo", scope: !531, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !532, templateParams: !13, identifier: "74943ad5cfeaa8d7c3439d6f603267a6")
!531 = !DINamespace(name: "panic_info", scope: !320)
!532 = !{!533, !534, !535, !536}
!533 = !DIDerivedType(tag: DW_TAG_member, name: "message", scope: !530, file: !2, baseType: !252, size: 32, align: 32, flags: DIFlagPrivate)
!534 = !DIDerivedType(tag: DW_TAG_member, name: "location", scope: !530, file: !2, baseType: !317, size: 32, align: 32, offset: 32, flags: DIFlagPrivate)
!535 = !DIDerivedType(tag: DW_TAG_member, name: "can_unwind", scope: !530, file: !2, baseType: !453, size: 8, align: 8, offset: 64, flags: DIFlagPrivate)
!536 = !DIDerivedType(tag: DW_TAG_member, name: "force_no_backtrace", scope: !530, file: !2, baseType: !453, size: 8, align: 8, offset: 72, flags: DIFlagPrivate)
!537 = !DILocation(line: 2423, column: 40, scope: !523)
!538 = !DILocation(line: 103, column: 17, scope: !539)
!539 = !DILexicalBlockFile(scope: !523, file: !510, discriminator: 0)
!540 = distinct !DISubprogram(name: "maybe_is_aligned", linkageName: "_ZN4core9ub_checks16maybe_is_aligned17haad94b0cc6d1077dE", scope: !541, file: !343, line: 135, type: !542, scopeLine: 135, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !544)
!541 = !DINamespace(name: "ub_checks", scope: !56)
!542 = !DISubroutineType(types: !543)
!543 = !{!453, !6, !9}
!544 = !{!545, !546}
!545 = !DILocalVariable(name: "ptr", arg: 1, scope: !540, file: !343, line: 135, type: !6)
!546 = !DILocalVariable(name: "align", arg: 2, scope: !540, file: !343, line: 135, type: !9)
!547 = !DILocation(line: 135, column: 38, scope: !540)
!548 = !DILocation(line: 135, column: 54, scope: !540)
!549 = !DILocation(line: 2435, column: 9, scope: !550)
!550 = !DILexicalBlockFile(scope: !540, file: !521, discriminator: 0)
!551 = !DILocation(line: 145, column: 2, scope: !540)
!552 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks16maybe_is_aligned7runtime17hda16d0ba0b9a56bdE", scope: !553, file: !521, line: 2423, type: !542, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !554)
!553 = !DINamespace(name: "maybe_is_aligned", scope: !541)
!554 = !{!555, !556}
!555 = !DILocalVariable(name: "ptr", arg: 1, scope: !552, file: !521, line: 2423, type: !6)
!556 = !DILocalVariable(name: "align", arg: 2, scope: !552, file: !521, line: 2423, type: !9)
!557 = !DILocation(line: 2423, column: 40, scope: !552)
!558 = !DILocation(line: 142, column: 17, scope: !559)
!559 = !DILexicalBlockFile(scope: !552, file: !343, discriminator: 0)
!560 = !DILocation(line: 2425, column: 10, scope: !552)
!561 = distinct !DISubprogram(name: "check_language_ub", linkageName: "_ZN4core9ub_checks17check_language_ub17h82985c394737b77cE", scope: !541, file: !343, line: 96, type: !562, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13)
!562 = !DISubroutineType(types: !563)
!563 = !{!453}
!564 = !DILocation(line: 98, column: 5, scope: !561)
!565 = !DILocation(line: 2435, column: 9, scope: !566)
!566 = !DILexicalBlockFile(scope: !561, file: !521, discriminator: 0)
!567 = !DILocation(line: 109, column: 2, scope: !561)
!568 = distinct !DISubprogram(name: "runtime", linkageName: "_ZN4core9ub_checks17check_language_ub7runtime17h0715ca3a72765d67E", scope: !569, file: !521, line: 2423, type: !562, scopeLine: 2423, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13)
!569 = !DINamespace(name: "check_language_ub", scope: !541)
!570 = !DILocation(line: 2425, column: 10, scope: !568)
!571 = distinct !DISubprogram(name: "maybe_is_aligned_and_not_null", linkageName: "_ZN4core9ub_checks29maybe_is_aligned_and_not_null17hc491a94493deec50E", scope: !541, file: !343, line: 119, type: !572, scopeLine: 119, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !574)
!572 = !DISubroutineType(types: !573)
!573 = !{!453, !6, !9, !453}
!574 = !{!575, !576, !577}
!575 = !DILocalVariable(name: "ptr", arg: 1, scope: !571, file: !343, line: 120, type: !6)
!576 = !DILocalVariable(name: "align", arg: 2, scope: !571, file: !343, line: 121, type: !9)
!577 = !DILocalVariable(name: "is_zst", arg: 3, scope: !571, file: !343, line: 122, type: !453)
!578 = !DILocation(line: 120, column: 5, scope: !571)
!579 = !DILocation(line: 121, column: 5, scope: !571)
!580 = !DILocation(line: 122, column: 5, scope: !571)
!581 = !DILocation(line: 125, column: 5, scope: !571)
!582 = !DILocation(line: 125, column: 38, scope: !571)
!583 = !DILocation(line: 126, column: 2, scope: !571)
!584 = !DILocation(line: 125, column: 53, scope: !571)
!585 = !DILocation(line: 125, column: 48, scope: !571)
!586 = !DILocation(line: 125, column: 37, scope: !571)
!587 = distinct !DISubprogram(name: "fd_readdir", linkageName: "_ZN4wasi13lib_generated10fd_readdir17h70dd87b0fd008942E", scope: !351, file: !588, line: 1598, type: !589, scopeLine: 1598, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !611)
!588 = !DIFile(filename: "src/lib_generated.rs", directory: "/Users/namse/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasi-0.11.1+wasi-snapshot-preview1", checksumkind: CSK_MD5, checksum: "547fc91ba6caf9be070898a050126dd3")
!589 = !DISubroutineType(types: !590)
!590 = !{!591, !44, !610, !9, !360}
!591 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<usize, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !592, templateParams: !13, identifier: "a141799c73d98e9ce0f447a6c3a399c")
!592 = !{!593}
!593 = !DICompositeType(tag: DW_TAG_variant_part, scope: !591, file: !2, size: 64, align: 32, elements: !594, templateParams: !13, identifier: "21f71d73271994a8ced188f0227c710", discriminator: !609)
!594 = !{!595, !605}
!595 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !593, file: !2, baseType: !596, size: 64, align: 32, extraData: i16 0)
!596 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !591, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !597, templateParams: !599, identifier: "76d15b3d5849a4f59c45609d11ba88f8")
!597 = !{!598}
!598 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !596, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!599 = !{!600, !601}
!600 = !DITemplateTypeParameter(name: "T", type: !9)
!601 = !DITemplateTypeParameter(name: "E", type: !602)
!602 = !DICompositeType(tag: DW_TAG_structure_type, name: "Errno", scope: !351, file: !2, size: 16, align: 16, flags: DIFlagPublic, elements: !603, templateParams: !13, identifier: "9b47191c9dac14bee2941ffcb3885eec")
!603 = !{!604}
!604 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !602, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagPrivate)
!605 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !593, file: !2, baseType: !606, size: 64, align: 32, extraData: i16 1)
!606 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !591, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !607, templateParams: !599, identifier: "395b16f805d648399b7de2da59fd8ce9")
!607 = !{!608}
!608 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !606, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!609 = !DIDerivedType(tag: DW_TAG_member, scope: !591, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!610 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!611 = !{!612, !613, !614, !615, !616, !629}
!612 = !DILocalVariable(name: "fd", arg: 1, scope: !587, file: !588, line: 1599, type: !44)
!613 = !DILocalVariable(name: "buf", arg: 2, scope: !587, file: !588, line: 1600, type: !610)
!614 = !DILocalVariable(name: "buf_len", arg: 3, scope: !587, file: !588, line: 1601, type: !9)
!615 = !DILocalVariable(name: "cookie", arg: 4, scope: !587, file: !588, line: 1602, type: !360)
!616 = !DILocalVariable(name: "rp0", scope: !617, file: !588, line: 1604, type: !618, align: 32)
!617 = distinct !DILexicalBlock(scope: !587, file: !588, line: 1604, column: 5)
!618 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<usize>", scope: !619, file: !2, size: 32, align: 32, elements: !621, templateParams: !628, identifier: "b8e0231bd6357640c8ea5a3bfea73185")
!619 = !DINamespace(name: "maybe_uninit", scope: !620)
!620 = !DINamespace(name: "mem", scope: !56)
!621 = !{!622, !623}
!622 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !618, file: !2, baseType: !7, align: 8)
!623 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !618, file: !2, baseType: !624, size: 32, align: 32)
!624 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<usize>", scope: !625, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !626, templateParams: !628, identifier: "cc5a7752ecfed6d271e3750e21422ca9")
!625 = !DINamespace(name: "manually_drop", scope: !620)
!626 = !{!627}
!627 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !624, file: !2, baseType: !9, size: 32, align: 32, flags: DIFlagPrivate)
!628 = !{!600}
!629 = !DILocalVariable(name: "ret", scope: !630, file: !588, line: 1605, type: !631, align: 32)
!630 = distinct !DILexicalBlock(scope: !617, file: !588, line: 1605, column: 5)
!631 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!632 = !DILocation(line: 1599, column: 5, scope: !587)
!633 = !DILocation(line: 1600, column: 5, scope: !587)
!634 = !DILocation(line: 1601, column: 5, scope: !587)
!635 = !DILocation(line: 1602, column: 5, scope: !587)
!636 = !DILocation(line: 1604, column: 9, scope: !617)
!637 = !DILocation(line: 1604, column: 19, scope: !587)
!638 = !DILocation(line: 1607, column: 9, scope: !617)
!639 = !DILocalVariable(name: "self", arg: 1, scope: !640, file: !641, line: 560, type: !645)
!640 = distinct !DISubprogram(name: "as_mut_ptr<usize>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h37c5e939d2090adfE", scope: !618, file: !641, line: 560, type: !642, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !628, declaration: !646, retainedNodes: !647)
!641 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs", directory: "", checksumkind: CSK_MD5, checksum: "6de2d108794a3cb7d570256a1615f222")
!642 = !DISubroutineType(types: !643)
!643 = !{!644, !645}
!644 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut usize", baseType: !9, size: 32, align: 32, dwarfAddressSpace: 0)
!645 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<usize>", baseType: !618, size: 32, align: 32, dwarfAddressSpace: 0)
!646 = !DISubprogram(name: "as_mut_ptr<usize>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h37c5e939d2090adfE", scope: !618, file: !641, line: 560, type: !642, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !628)
!647 = !{!639}
!648 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !649)
!649 = distinct !DILocation(line: 1610, column: 13, scope: !617)
!650 = !DILocation(line: 1610, column: 9, scope: !617)
!651 = !DILocation(line: 1605, column: 15, scope: !617)
!652 = !DILocation(line: 1605, column: 9, scope: !630)
!653 = !DILocation(line: 1612, column: 5, scope: !630)
!654 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !655)
!655 = distinct !DILocation(line: 1613, column: 37, scope: !630)
!656 = !DILocation(line: 1613, column: 33, scope: !630)
!657 = !DILocation(line: 1613, column: 17, scope: !630)
!658 = !DILocation(line: 1613, column: 14, scope: !630)
!659 = !DILocation(line: 1613, column: 72, scope: !630)
!660 = !DILocation(line: 1614, column: 24, scope: !630)
!661 = !DILocation(line: 1614, column: 14, scope: !630)
!662 = !DILocation(line: 1614, column: 35, scope: !630)
!663 = !DILocation(line: 1616, column: 2, scope: !587)
!664 = distinct !DISubprogram(name: "proc_raise", linkageName: "_ZN4wasi13lib_generated10proc_raise17h7a2e0aeb3b0e2c49E", scope: !351, file: !588, line: 2028, type: !665, scopeLine: 2028, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !684)
!665 = !DISubroutineType(types: !666)
!666 = !{!667, !681}
!667 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), wasi::lib_generated::Errno>", scope: !60, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !668, templateParams: !13, identifier: "6609f8701d6c58f07153c01845c109bc")
!668 = !{!669}
!669 = !DICompositeType(tag: DW_TAG_variant_part, scope: !667, file: !2, size: 32, align: 16, elements: !670, templateParams: !13, identifier: "58f8948c7aba808f2158d9e06fc9f4e6", discriminator: !680)
!670 = !{!671, !676}
!671 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !669, file: !2, baseType: !672, size: 32, align: 16, extraData: i16 0)
!672 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !667, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !673, templateParams: !675, identifier: "9a33842f7e0505dfdd150216872198fb")
!673 = !{!674}
!674 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !672, file: !2, baseType: !7, align: 8, offset: 16, flags: DIFlagPublic)
!675 = !{!69, !601}
!676 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !669, file: !2, baseType: !677, size: 32, align: 16, extraData: i16 1)
!677 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !667, file: !2, size: 32, align: 16, flags: DIFlagPublic, elements: !678, templateParams: !675, identifier: "10519c1708c25086ccf5ab779f05f03c")
!678 = !{!679}
!679 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !677, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!680 = !DIDerivedType(tag: DW_TAG_member, scope: !667, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!681 = !DICompositeType(tag: DW_TAG_structure_type, name: "Signal", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !682, templateParams: !13, identifier: "21b14c6948fdf80c2ae268fb6ace6e47")
!682 = !{!683}
!683 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !681, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!684 = !{!685, !686}
!685 = !DILocalVariable(name: "sig", arg: 1, scope: !664, file: !588, line: 2028, type: !681)
!686 = !DILocalVariable(name: "ret", scope: !687, file: !588, line: 2029, type: !631, align: 32)
!687 = distinct !DILexicalBlock(scope: !664, file: !588, line: 2029, column: 5)
!688 = !DILocation(line: 2028, column: 26, scope: !664)
!689 = !DILocation(line: 2029, column: 50, scope: !664)
!690 = !DILocation(line: 2029, column: 15, scope: !664)
!691 = !DILocation(line: 2029, column: 9, scope: !687)
!692 = !DILocation(line: 2030, column: 5, scope: !687)
!693 = !DILocation(line: 2031, column: 14, scope: !687)
!694 = !DILocation(line: 2031, column: 19, scope: !687)
!695 = !DILocation(line: 2032, column: 24, scope: !687)
!696 = !DILocation(line: 2032, column: 14, scope: !687)
!697 = !DILocation(line: 2032, column: 35, scope: !687)
!698 = !DILocation(line: 2034, column: 2, scope: !664)
!699 = distinct !DISubprogram(name: "random_get", linkageName: "_ZN4wasi13lib_generated10random_get17h623dfe08f2cd86edE", scope: !351, file: !588, line: 2056, type: !700, scopeLine: 2056, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !702)
!700 = !DISubroutineType(types: !701)
!701 = !{!667, !610, !9}
!702 = !{!703, !704, !705}
!703 = !DILocalVariable(name: "buf", arg: 1, scope: !699, file: !588, line: 2056, type: !610)
!704 = !DILocalVariable(name: "buf_len", arg: 2, scope: !699, file: !588, line: 2056, type: !9)
!705 = !DILocalVariable(name: "ret", scope: !706, file: !588, line: 2057, type: !631, align: 32)
!706 = distinct !DILexicalBlock(scope: !699, file: !588, line: 2057, column: 5)
!707 = !DILocation(line: 2056, column: 26, scope: !699)
!708 = !DILocation(line: 2056, column: 40, scope: !699)
!709 = !DILocation(line: 2057, column: 50, scope: !699)
!710 = !DILocation(line: 2057, column: 15, scope: !699)
!711 = !DILocation(line: 2057, column: 9, scope: !706)
!712 = !DILocation(line: 2058, column: 5, scope: !706)
!713 = !DILocation(line: 2059, column: 14, scope: !706)
!714 = !DILocation(line: 2059, column: 19, scope: !706)
!715 = !DILocation(line: 2060, column: 24, scope: !706)
!716 = !DILocation(line: 2060, column: 14, scope: !706)
!717 = !DILocation(line: 2060, column: 35, scope: !706)
!718 = !DILocation(line: 2062, column: 2, scope: !699)
!719 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated11Preopentype3raw17h66bac64334fbea7cE", scope: !720, file: !588, line: 1163, type: !723, scopeLine: 1163, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !726, retainedNodes: !727)
!720 = !DICompositeType(tag: DW_TAG_structure_type, name: "Preopentype", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !721, templateParams: !13, identifier: "610b2c516912183380c929386866152b")
!721 = !{!722}
!722 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !720, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!723 = !DISubroutineType(types: !724)
!724 = !{!26, !725}
!725 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Preopentype", baseType: !720, size: 32, align: 32, dwarfAddressSpace: 0)
!726 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated11Preopentype3raw17h66bac64334fbea7cE", scope: !720, file: !588, line: 1163, type: !723, scopeLine: 1163, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!727 = !{!728}
!728 = !DILocalVariable(name: "self", arg: 1, scope: !719, file: !588, line: 1163, type: !725)
!729 = !DILocation(line: 1163, column: 22, scope: !719)
!730 = !DILocation(line: 1164, column: 9, scope: !719)
!731 = !DILocation(line: 1165, column: 6, scope: !719)
!732 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated11Preopentype4name17h51a1ca3954e9478cE", scope: !720, file: !588, line: 1167, type: !733, scopeLine: 1167, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !735, retainedNodes: !736)
!733 = !DISubroutineType(types: !734)
!734 = !{!22, !725}
!735 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated11Preopentype4name17h51a1ca3954e9478cE", scope: !720, file: !588, line: 1167, type: !733, scopeLine: 1167, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!736 = !{!737}
!737 = !DILocalVariable(name: "self", arg: 1, scope: !732, file: !588, line: 1167, type: !725)
!738 = !DILocation(line: 1167, column: 17, scope: !732)
!739 = !DILocation(line: 1168, column: 9, scope: !732)
!740 = !DILocation(line: 1172, column: 6, scope: !732)
!741 = !DILocation(line: 1170, column: 27, scope: !732)
!742 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated11Preopentype7message17ha683a318ca855ec8E", scope: !720, file: !588, line: 1173, type: !733, scopeLine: 1173, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !743, retainedNodes: !744)
!743 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated11Preopentype7message17ha683a318ca855ec8E", scope: !720, file: !588, line: 1173, type: !733, scopeLine: 1173, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!744 = !{!745}
!745 = !DILocalVariable(name: "self", arg: 1, scope: !742, file: !588, line: 1173, type: !725)
!746 = !DILocation(line: 1173, column: 20, scope: !742)
!747 = !DILocation(line: 1174, column: 9, scope: !742)
!748 = !DILocation(line: 1178, column: 6, scope: !742)
!749 = !DILocation(line: 1176, column: 27, scope: !742)
!750 = distinct !DISubprogram(name: "environ_get", linkageName: "_ZN4wasi13lib_generated11environ_get17he9729f8c36e4c46eE", scope: !351, file: !588, line: 1242, type: !751, scopeLine: 1242, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !754)
!751 = !DISubroutineType(types: !752)
!752 = !{!667, !753, !610}
!753 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut *mut u8", baseType: !610, size: 32, align: 32, dwarfAddressSpace: 0)
!754 = !{!755, !756, !757}
!755 = !DILocalVariable(name: "environ", arg: 1, scope: !750, file: !588, line: 1242, type: !753)
!756 = !DILocalVariable(name: "environ_buf", arg: 2, scope: !750, file: !588, line: 1242, type: !610)
!757 = !DILocalVariable(name: "ret", scope: !758, file: !588, line: 1243, type: !631, align: 32)
!758 = distinct !DILexicalBlock(scope: !750, file: !588, line: 1243, column: 5)
!759 = !DILocation(line: 1242, column: 27, scope: !750)
!760 = !DILocation(line: 1242, column: 50, scope: !750)
!761 = !DILocation(line: 1243, column: 51, scope: !750)
!762 = !DILocation(line: 1243, column: 67, scope: !750)
!763 = !DILocation(line: 1243, column: 15, scope: !750)
!764 = !DILocation(line: 1243, column: 9, scope: !758)
!765 = !DILocation(line: 1244, column: 5, scope: !758)
!766 = !DILocation(line: 1245, column: 14, scope: !758)
!767 = !DILocation(line: 1245, column: 19, scope: !758)
!768 = !DILocation(line: 1246, column: 24, scope: !758)
!769 = !DILocation(line: 1246, column: 14, scope: !758)
!770 = !DILocation(line: 1246, column: 35, scope: !758)
!771 = !DILocation(line: 1248, column: 2, scope: !750)
!772 = distinct !DISubprogram(name: "fd_allocate", linkageName: "_ZN4wasi13lib_generated11fd_allocate17hf95871a71c82ef1eE", scope: !351, file: !588, line: 1344, type: !773, scopeLine: 1344, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !775)
!773 = !DISubroutineType(types: !774)
!774 = !{!667, !44, !360, !360}
!775 = !{!776, !777, !778, !779}
!776 = !DILocalVariable(name: "fd", arg: 1, scope: !772, file: !588, line: 1344, type: !44)
!777 = !DILocalVariable(name: "offset", arg: 2, scope: !772, file: !588, line: 1344, type: !360)
!778 = !DILocalVariable(name: "len", arg: 3, scope: !772, file: !588, line: 1344, type: !360)
!779 = !DILocalVariable(name: "ret", scope: !780, file: !588, line: 1345, type: !631, align: 32)
!780 = distinct !DILexicalBlock(scope: !772, file: !588, line: 1345, column: 5)
!781 = !DILocation(line: 1344, column: 27, scope: !772)
!782 = !DILocation(line: 1344, column: 35, scope: !772)
!783 = !DILocation(line: 1344, column: 53, scope: !772)
!784 = !DILocation(line: 1345, column: 15, scope: !772)
!785 = !DILocation(line: 1345, column: 9, scope: !780)
!786 = !DILocation(line: 1346, column: 5, scope: !780)
!787 = !DILocation(line: 1347, column: 14, scope: !780)
!788 = !DILocation(line: 1347, column: 19, scope: !780)
!789 = !DILocation(line: 1348, column: 24, scope: !780)
!790 = !DILocation(line: 1348, column: 14, scope: !780)
!791 = !DILocation(line: 1348, column: 35, scope: !780)
!792 = !DILocation(line: 1350, column: 2, scope: !772)
!793 = distinct !DISubprogram(name: "fd_datasync", linkageName: "_ZN4wasi13lib_generated11fd_datasync17h896903f285885e8dE", scope: !351, file: !588, line: 1364, type: !794, scopeLine: 1364, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !796)
!794 = !DISubroutineType(types: !795)
!795 = !{!667, !44}
!796 = !{!797, !798}
!797 = !DILocalVariable(name: "fd", arg: 1, scope: !793, file: !588, line: 1364, type: !44)
!798 = !DILocalVariable(name: "ret", scope: !799, file: !588, line: 1365, type: !631, align: 32)
!799 = distinct !DILexicalBlock(scope: !793, file: !588, line: 1365, column: 5)
!800 = !DILocation(line: 1364, column: 27, scope: !793)
!801 = !DILocation(line: 1365, column: 15, scope: !793)
!802 = !DILocation(line: 1365, column: 9, scope: !799)
!803 = !DILocation(line: 1366, column: 5, scope: !799)
!804 = !DILocation(line: 1367, column: 14, scope: !799)
!805 = !DILocation(line: 1367, column: 19, scope: !799)
!806 = !DILocation(line: 1368, column: 24, scope: !799)
!807 = !DILocation(line: 1368, column: 14, scope: !799)
!808 = !DILocation(line: 1368, column: 35, scope: !799)
!809 = !DILocation(line: 1370, column: 2, scope: !793)
!810 = distinct !DISubprogram(name: "fd_renumber", linkageName: "_ZN4wasi13lib_generated11fd_renumber17ha35f9110cd723dc5E", scope: !351, file: !588, line: 1630, type: !811, scopeLine: 1630, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !813)
!811 = !DISubroutineType(types: !812)
!812 = !{!667, !44, !44}
!813 = !{!814, !815, !816}
!814 = !DILocalVariable(name: "fd", arg: 1, scope: !810, file: !588, line: 1630, type: !44)
!815 = !DILocalVariable(name: "to", arg: 2, scope: !810, file: !588, line: 1630, type: !44)
!816 = !DILocalVariable(name: "ret", scope: !817, file: !588, line: 1631, type: !631, align: 32)
!817 = distinct !DILexicalBlock(scope: !810, file: !588, line: 1631, column: 5)
!818 = !DILocation(line: 1630, column: 27, scope: !810)
!819 = !DILocation(line: 1630, column: 35, scope: !810)
!820 = !DILocation(line: 1631, column: 15, scope: !810)
!821 = !DILocation(line: 1631, column: 9, scope: !817)
!822 = !DILocation(line: 1632, column: 5, scope: !817)
!823 = !DILocation(line: 1633, column: 14, scope: !817)
!824 = !DILocation(line: 1633, column: 19, scope: !817)
!825 = !DILocation(line: 1634, column: 24, scope: !817)
!826 = !DILocation(line: 1634, column: 14, scope: !817)
!827 = !DILocation(line: 1634, column: 35, scope: !817)
!828 = !DILocation(line: 1636, column: 2, scope: !810)
!829 = distinct !DISubprogram(name: "path_rename", linkageName: "_ZN4wasi13lib_generated11path_rename17hecc7b81cf9ee57abE", scope: !351, file: !588, line: 1927, type: !830, scopeLine: 1927, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !832)
!830 = !DISubroutineType(types: !831)
!831 = !{!667, !44, !22, !44, !22}
!832 = !{!833, !834, !835, !836, !837}
!833 = !DILocalVariable(name: "fd", arg: 1, scope: !829, file: !588, line: 1927, type: !44)
!834 = !DILocalVariable(name: "old_path", arg: 2, scope: !829, file: !588, line: 1927, type: !22)
!835 = !DILocalVariable(name: "new_fd", arg: 3, scope: !829, file: !588, line: 1927, type: !44)
!836 = !DILocalVariable(name: "new_path", arg: 4, scope: !829, file: !588, line: 1927, type: !22)
!837 = !DILocalVariable(name: "ret", scope: !838, file: !588, line: 1928, type: !631, align: 32)
!838 = distinct !DILexicalBlock(scope: !829, file: !588, line: 1928, column: 5)
!839 = !DILocation(line: 1927, column: 27, scope: !829)
!840 = !DILocation(line: 1927, column: 35, scope: !829)
!841 = !DILocation(line: 1927, column: 51, scope: !829)
!842 = !DILocation(line: 1927, column: 63, scope: !829)
!843 = !DILocalVariable(name: "self", arg: 1, scope: !844, file: !469, line: 562, type: !22)
!844 = distinct !DISubprogram(name: "as_ptr", linkageName: "_ZN4core3str21_$LT$impl$u20$str$GT$6as_ptr17h9cdaf2fb3e15fe28E", scope: !470, file: !469, line: 562, type: !845, scopeLine: 562, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !848)
!845 = !DISubroutineType(types: !846)
!846 = !{!847, !22}
!847 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !26, size: 32, align: 32, dwarfAddressSpace: 0)
!848 = !{!843}
!849 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !850)
!850 = distinct !DILocation(line: 1930, column: 18, scope: !829)
!851 = !DILocation(line: 1930, column: 9, scope: !829)
!852 = !DILocation(line: 1931, column: 18, scope: !829)
!853 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !854)
!854 = distinct !DILocation(line: 1933, column: 18, scope: !829)
!855 = !DILocation(line: 1933, column: 9, scope: !829)
!856 = !DILocation(line: 1934, column: 18, scope: !829)
!857 = !DILocation(line: 1928, column: 15, scope: !829)
!858 = !DILocation(line: 1928, column: 9, scope: !838)
!859 = !DILocation(line: 1936, column: 5, scope: !838)
!860 = !DILocation(line: 1937, column: 14, scope: !838)
!861 = !DILocation(line: 1937, column: 19, scope: !838)
!862 = !DILocation(line: 1938, column: 24, scope: !838)
!863 = !DILocation(line: 1938, column: 14, scope: !838)
!864 = !DILocation(line: 1938, column: 35, scope: !838)
!865 = !DILocation(line: 1940, column: 2, scope: !829)
!866 = distinct !DISubprogram(name: "poll_oneoff", linkageName: "_ZN4wasi13lib_generated11poll_oneoff17hb1a91275e741c2fdE", scope: !351, file: !588, line: 1993, type: !867, scopeLine: 1993, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !909)
!867 = !DISubroutineType(types: !868)
!868 = !{!591, !869, !895, !9}
!869 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Subscription", baseType: !870, size: 32, align: 32, dwarfAddressSpace: 0)
!870 = !DICompositeType(tag: DW_TAG_structure_type, name: "Subscription", scope: !351, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !871, templateParams: !13, identifier: "8da8c210b66aba2645fb9bd7db578ec8")
!871 = !{!872, !873}
!872 = !DIDerivedType(tag: DW_TAG_member, name: "userdata", scope: !870, file: !2, baseType: !360, size: 64, align: 64, flags: DIFlagPublic)
!873 = !DIDerivedType(tag: DW_TAG_member, name: "u", scope: !870, file: !2, baseType: !874, size: 320, align: 64, offset: 64, flags: DIFlagPublic)
!874 = !DICompositeType(tag: DW_TAG_structure_type, name: "SubscriptionU", scope: !351, file: !2, size: 320, align: 64, flags: DIFlagPublic, elements: !875, templateParams: !13, identifier: "b603683e2e0919d17609e65d162ccf16")
!875 = !{!876, !877}
!876 = !DIDerivedType(tag: DW_TAG_member, name: "tag", scope: !874, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPublic)
!877 = !DIDerivedType(tag: DW_TAG_member, name: "u", scope: !874, file: !2, baseType: !878, size: 256, align: 64, offset: 64, flags: DIFlagPublic)
!878 = !DICompositeType(tag: DW_TAG_union_type, name: "SubscriptionUU", scope: !351, file: !2, size: 256, align: 64, elements: !879, templateParams: !13, identifier: "b969325dc869ec8e94bdb44672666040")
!879 = !{!880, !890, !894}
!880 = !DIDerivedType(tag: DW_TAG_member, name: "clock", scope: !878, file: !2, baseType: !881, size: 256, align: 64)
!881 = !DICompositeType(tag: DW_TAG_structure_type, name: "SubscriptionClock", scope: !351, file: !2, size: 256, align: 64, flags: DIFlagPublic, elements: !882, templateParams: !13, identifier: "28a76cae444232a65cdf4ce20628cf9b")
!882 = !{!883, !887, !888, !889}
!883 = !DIDerivedType(tag: DW_TAG_member, name: "id", scope: !881, file: !2, baseType: !884, size: 32, align: 32, flags: DIFlagPublic)
!884 = !DICompositeType(tag: DW_TAG_structure_type, name: "Clockid", scope: !351, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !885, templateParams: !13, identifier: "402969a54c50692a7a19965fab563a54")
!885 = !{!886}
!886 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !884, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagPrivate)
!887 = !DIDerivedType(tag: DW_TAG_member, name: "timeout", scope: !881, file: !2, baseType: !360, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!888 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !881, file: !2, baseType: !360, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
!889 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !881, file: !2, baseType: !12, size: 16, align: 16, offset: 192, flags: DIFlagPublic)
!890 = !DIDerivedType(tag: DW_TAG_member, name: "fd_read", scope: !878, file: !2, baseType: !891, size: 32, align: 32)
!891 = !DICompositeType(tag: DW_TAG_structure_type, name: "SubscriptionFdReadwrite", scope: !351, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !892, templateParams: !13, identifier: "d490a63562d6df3eb5451158e24fea24")
!892 = !{!893}
!893 = !DIDerivedType(tag: DW_TAG_member, name: "file_descriptor", scope: !891, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagPublic)
!894 = !DIDerivedType(tag: DW_TAG_member, name: "fd_write", scope: !878, file: !2, baseType: !891, size: 32, align: 32)
!895 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut wasi::lib_generated::Event", baseType: !896, size: 32, align: 32, dwarfAddressSpace: 0)
!896 = !DICompositeType(tag: DW_TAG_structure_type, name: "Event", scope: !351, file: !2, size: 256, align: 64, flags: DIFlagPublic, elements: !897, templateParams: !13, identifier: "2c21f2ac42b08edef0df6aab055d94eb")
!897 = !{!898, !899, !900, !904}
!898 = !DIDerivedType(tag: DW_TAG_member, name: "userdata", scope: !896, file: !2, baseType: !360, size: 64, align: 64, flags: DIFlagPublic)
!899 = !DIDerivedType(tag: DW_TAG_member, name: "error", scope: !896, file: !2, baseType: !602, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
!900 = !DIDerivedType(tag: DW_TAG_member, name: "type_", scope: !896, file: !2, baseType: !901, size: 8, align: 8, offset: 80, flags: DIFlagPublic)
!901 = !DICompositeType(tag: DW_TAG_structure_type, name: "Eventtype", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !902, templateParams: !13, identifier: "d36064fdb246235c9b3f43a829bd7fc7")
!902 = !{!903}
!903 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !901, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!904 = !DIDerivedType(tag: DW_TAG_member, name: "fd_readwrite", scope: !896, file: !2, baseType: !905, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!905 = !DICompositeType(tag: DW_TAG_structure_type, name: "EventFdReadwrite", scope: !351, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !906, templateParams: !13, identifier: "6381a9db7eda7d05f17187494acc08e4")
!906 = !{!907, !908}
!907 = !DIDerivedType(tag: DW_TAG_member, name: "nbytes", scope: !905, file: !2, baseType: !360, size: 64, align: 64, flags: DIFlagPublic)
!908 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !905, file: !2, baseType: !12, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
!909 = !{!910, !911, !912, !913, !915}
!910 = !DILocalVariable(name: "in_", arg: 1, scope: !866, file: !588, line: 1994, type: !869)
!911 = !DILocalVariable(name: "out", arg: 2, scope: !866, file: !588, line: 1995, type: !895)
!912 = !DILocalVariable(name: "nsubscriptions", arg: 3, scope: !866, file: !588, line: 1996, type: !9)
!913 = !DILocalVariable(name: "rp0", scope: !914, file: !588, line: 1998, type: !618, align: 32)
!914 = distinct !DILexicalBlock(scope: !866, file: !588, line: 1998, column: 5)
!915 = !DILocalVariable(name: "ret", scope: !916, file: !588, line: 1999, type: !631, align: 32)
!916 = distinct !DILexicalBlock(scope: !914, file: !588, line: 1999, column: 5)
!917 = !DILocation(line: 1994, column: 5, scope: !866)
!918 = !DILocation(line: 1995, column: 5, scope: !866)
!919 = !DILocation(line: 1996, column: 5, scope: !866)
!920 = !DILocation(line: 1998, column: 9, scope: !914)
!921 = !DILocation(line: 1998, column: 19, scope: !866)
!922 = !DILocation(line: 2000, column: 9, scope: !914)
!923 = !DILocation(line: 2001, column: 9, scope: !914)
!924 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !925)
!925 = distinct !DILocation(line: 2003, column: 13, scope: !914)
!926 = !DILocation(line: 2003, column: 9, scope: !914)
!927 = !DILocation(line: 1999, column: 15, scope: !914)
!928 = !DILocation(line: 1999, column: 9, scope: !916)
!929 = !DILocation(line: 2005, column: 5, scope: !916)
!930 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !931)
!931 = distinct !DILocation(line: 2006, column: 37, scope: !916)
!932 = !DILocation(line: 2006, column: 33, scope: !916)
!933 = !DILocation(line: 2006, column: 17, scope: !916)
!934 = !DILocation(line: 2006, column: 14, scope: !916)
!935 = !DILocation(line: 2006, column: 72, scope: !916)
!936 = !DILocation(line: 2007, column: 24, scope: !916)
!937 = !DILocation(line: 2007, column: 14, scope: !916)
!938 = !DILocation(line: 2007, column: 35, scope: !916)
!939 = !DILocation(line: 2009, column: 2, scope: !866)
!940 = distinct !DISubprogram(name: "sched_yield", linkageName: "_ZN4wasi13lib_generated11sched_yield17h754331c024719597E", scope: !351, file: !588, line: 2038, type: !941, scopeLine: 2038, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !943)
!941 = !DISubroutineType(types: !942)
!942 = !{!667}
!943 = !{!944}
!944 = !DILocalVariable(name: "ret", scope: !945, file: !588, line: 2039, type: !631, align: 32)
!945 = distinct !DILexicalBlock(scope: !940, file: !588, line: 2039, column: 5)
!946 = !DILocation(line: 2039, column: 15, scope: !940)
!947 = !DILocation(line: 2039, column: 9, scope: !945)
!948 = !DILocation(line: 2040, column: 5, scope: !945)
!949 = !DILocation(line: 2041, column: 14, scope: !945)
!950 = !DILocation(line: 2041, column: 19, scope: !945)
!951 = !DILocation(line: 2042, column: 24, scope: !945)
!952 = !DILocation(line: 2042, column: 14, scope: !945)
!953 = !DILocation(line: 2042, column: 35, scope: !945)
!954 = !DILocation(line: 2044, column: 2, scope: !940)
!955 = distinct !DISubprogram(name: "sock_accept", linkageName: "_ZN4wasi13lib_generated11sock_accept17h300d2bf8b4b573d3E", scope: !351, file: !588, line: 2075, type: !956, scopeLine: 2075, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !972)
!956 = !DISubroutineType(types: !957)
!957 = !{!958, !44, !12}
!958 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<u32, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !959, templateParams: !13, identifier: "b9f1b5c924c18c3252161cf42a04033e")
!959 = !{!960}
!960 = !DICompositeType(tag: DW_TAG_variant_part, scope: !958, file: !2, size: 64, align: 32, elements: !961, templateParams: !13, identifier: "408e84fd462e51153ab1b6742dc9b442", discriminator: !971)
!961 = !{!962, !967}
!962 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !960, file: !2, baseType: !963, size: 64, align: 32, extraData: i16 0)
!963 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !958, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !964, templateParams: !966, identifier: "821bc9d2c37c0e42db97994a40fa8a3a")
!964 = !{!965}
!965 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !963, file: !2, baseType: !44, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!966 = !{!442, !601}
!967 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !960, file: !2, baseType: !968, size: 64, align: 32, extraData: i16 1)
!968 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !958, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !969, templateParams: !966, identifier: "1d3f41cca37d2241221953e5414f22cc")
!969 = !{!970}
!970 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !968, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!971 = !DIDerivedType(tag: DW_TAG_member, scope: !958, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!972 = !{!973, !974, !975, !984}
!973 = !DILocalVariable(name: "fd", arg: 1, scope: !955, file: !588, line: 2075, type: !44)
!974 = !DILocalVariable(name: "flags", arg: 2, scope: !955, file: !588, line: 2075, type: !12)
!975 = !DILocalVariable(name: "rp0", scope: !976, file: !588, line: 2076, type: !977, align: 32)
!976 = distinct !DILexicalBlock(scope: !955, file: !588, line: 2076, column: 5)
!977 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u32>", scope: !619, file: !2, size: 32, align: 32, elements: !978, templateParams: !441, identifier: "52a5877638ce66e2347ecc7dd79ec760")
!978 = !{!979, !980}
!979 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !977, file: !2, baseType: !7, align: 8)
!980 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !977, file: !2, baseType: !981, size: 32, align: 32)
!981 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u32>", scope: !625, file: !2, size: 32, align: 32, flags: DIFlagPublic, elements: !982, templateParams: !441, identifier: "3280d801f801104734b467ad49dc7394")
!982 = !{!983}
!983 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !981, file: !2, baseType: !44, size: 32, align: 32, flags: DIFlagPrivate)
!984 = !DILocalVariable(name: "ret", scope: !985, file: !588, line: 2077, type: !631, align: 32)
!985 = distinct !DILexicalBlock(scope: !976, file: !588, line: 2077, column: 5)
!986 = !DILocation(line: 2075, column: 27, scope: !955)
!987 = !DILocation(line: 2075, column: 35, scope: !955)
!988 = !DILocation(line: 2076, column: 9, scope: !976)
!989 = !DILocation(line: 2076, column: 19, scope: !955)
!990 = !DILocation(line: 2077, column: 62, scope: !976)
!991 = !DILocalVariable(name: "self", arg: 1, scope: !992, file: !641, line: 560, type: !996)
!992 = distinct !DISubprogram(name: "as_mut_ptr<u32>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h266e1c70e6db7950E", scope: !977, file: !641, line: 560, type: !993, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !441, declaration: !997, retainedNodes: !998)
!993 = !DISubroutineType(types: !994)
!994 = !{!995, !996}
!995 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u32", baseType: !44, size: 32, align: 32, dwarfAddressSpace: 0)
!996 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<u32>", baseType: !977, size: 32, align: 32, dwarfAddressSpace: 0)
!997 = !DISubprogram(name: "as_mut_ptr<u32>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h266e1c70e6db7950E", scope: !977, file: !641, line: 560, type: !993, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !441)
!998 = !{!991}
!999 = !DILocation(line: 560, column: 29, scope: !992, inlinedAt: !1000)
!1000 = distinct !DILocation(line: 2077, column: 80, scope: !976)
!1001 = !DILocation(line: 2077, column: 76, scope: !976)
!1002 = !DILocation(line: 2077, column: 15, scope: !976)
!1003 = !DILocation(line: 2077, column: 9, scope: !985)
!1004 = !DILocation(line: 2078, column: 5, scope: !985)
!1005 = !DILocation(line: 560, column: 29, scope: !992, inlinedAt: !1006)
!1006 = distinct !DILocation(line: 2079, column: 37, scope: !985)
!1007 = !DILocation(line: 2079, column: 33, scope: !985)
!1008 = !DILocation(line: 2079, column: 17, scope: !985)
!1009 = !DILocation(line: 2079, column: 14, scope: !985)
!1010 = !DILocation(line: 2079, column: 70, scope: !985)
!1011 = !DILocation(line: 2080, column: 24, scope: !985)
!1012 = !DILocation(line: 2080, column: 14, scope: !985)
!1013 = !DILocation(line: 2080, column: 35, scope: !985)
!1014 = !DILocation(line: 2082, column: 2, scope: !955)
!1015 = distinct !DISubprogram(name: "path_symlink", linkageName: "_ZN4wasi13lib_generated12path_symlink17h382523bc6931237dE", scope: !351, file: !588, line: 1949, type: !1016, scopeLine: 1949, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1018)
!1016 = !DISubroutineType(types: !1017)
!1017 = !{!667, !22, !44, !22}
!1018 = !{!1019, !1020, !1021, !1022}
!1019 = !DILocalVariable(name: "old_path", arg: 1, scope: !1015, file: !588, line: 1949, type: !22)
!1020 = !DILocalVariable(name: "fd", arg: 2, scope: !1015, file: !588, line: 1949, type: !44)
!1021 = !DILocalVariable(name: "new_path", arg: 3, scope: !1015, file: !588, line: 1949, type: !22)
!1022 = !DILocalVariable(name: "ret", scope: !1023, file: !588, line: 1950, type: !631, align: 32)
!1023 = distinct !DILexicalBlock(scope: !1015, file: !588, line: 1950, column: 5)
!1024 = !DILocation(line: 1949, column: 28, scope: !1015)
!1025 = !DILocation(line: 1949, column: 44, scope: !1015)
!1026 = !DILocation(line: 1949, column: 52, scope: !1015)
!1027 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1028)
!1028 = distinct !DILocation(line: 1951, column: 18, scope: !1015)
!1029 = !DILocation(line: 1951, column: 9, scope: !1015)
!1030 = !DILocation(line: 1952, column: 18, scope: !1015)
!1031 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1032)
!1032 = distinct !DILocation(line: 1954, column: 18, scope: !1015)
!1033 = !DILocation(line: 1954, column: 9, scope: !1015)
!1034 = !DILocation(line: 1955, column: 18, scope: !1015)
!1035 = !DILocation(line: 1950, column: 15, scope: !1015)
!1036 = !DILocation(line: 1950, column: 9, scope: !1023)
!1037 = !DILocation(line: 1957, column: 5, scope: !1023)
!1038 = !DILocation(line: 1958, column: 14, scope: !1023)
!1039 = !DILocation(line: 1958, column: 19, scope: !1023)
!1040 = !DILocation(line: 1959, column: 24, scope: !1023)
!1041 = !DILocation(line: 1959, column: 14, scope: !1023)
!1042 = !DILocation(line: 1959, column: 35, scope: !1023)
!1043 = !DILocation(line: 1961, column: 2, scope: !1015)
!1044 = distinct !DISubprogram(name: "clock_res_get", linkageName: "_ZN4wasi13lib_generated13clock_res_get17h963bdaa4c47e82c7E", scope: !351, file: !588, line: 1282, type: !1045, scopeLine: 1282, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1061)
!1045 = !DISubroutineType(types: !1046)
!1046 = !{!1047, !884}
!1047 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<u64, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1048, templateParams: !13, identifier: "37c446e14b242af26734d99f94ae6d8f")
!1048 = !{!1049}
!1049 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1047, file: !2, size: 128, align: 64, elements: !1050, templateParams: !13, identifier: "fb99b26091e543d7811729574a73b22e", discriminator: !1060)
!1050 = !{!1051, !1056}
!1051 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1049, file: !2, baseType: !1052, size: 128, align: 64, extraData: i16 0)
!1052 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1047, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1053, templateParams: !1055, identifier: "b310aac13efd76349438379fd42924e2")
!1053 = !{!1054}
!1054 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1052, file: !2, baseType: !360, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!1055 = !{!380, !601}
!1056 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1049, file: !2, baseType: !1057, size: 128, align: 64, extraData: i16 1)
!1057 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1047, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1058, templateParams: !1055, identifier: "1c69838a2534631ca1d0afb6caf22a26")
!1058 = !{!1059}
!1059 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1057, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1060 = !DIDerivedType(tag: DW_TAG_member, scope: !1047, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!1061 = !{!1062, !1063, !1072}
!1062 = !DILocalVariable(name: "id", arg: 1, scope: !1044, file: !588, line: 1282, type: !884)
!1063 = !DILocalVariable(name: "rp0", scope: !1064, file: !588, line: 1283, type: !1065, align: 64)
!1064 = distinct !DILexicalBlock(scope: !1044, file: !588, line: 1283, column: 5)
!1065 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u64>", scope: !619, file: !2, size: 64, align: 64, elements: !1066, templateParams: !379, identifier: "ad853c86c35c91bbf7836739e76f6a44")
!1066 = !{!1067, !1068}
!1067 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !1065, file: !2, baseType: !7, align: 8)
!1068 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1065, file: !2, baseType: !1069, size: 64, align: 64)
!1069 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u64>", scope: !625, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1070, templateParams: !379, identifier: "c5ae4d98ac6d63094ddf1e8ac6cd22d2")
!1070 = !{!1071}
!1071 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1069, file: !2, baseType: !360, size: 64, align: 64, flags: DIFlagPrivate)
!1072 = !DILocalVariable(name: "ret", scope: !1073, file: !588, line: 1284, type: !631, align: 32)
!1073 = distinct !DILexicalBlock(scope: !1064, file: !588, line: 1284, column: 5)
!1074 = !DILocation(line: 1282, column: 29, scope: !1044)
!1075 = !DILocation(line: 1283, column: 9, scope: !1064)
!1076 = !DILocation(line: 1283, column: 19, scope: !1044)
!1077 = !DILocalVariable(name: "self", arg: 1, scope: !1078, file: !641, line: 560, type: !1082)
!1078 = distinct !DISubprogram(name: "as_mut_ptr<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17he45af68aefaeac8fE", scope: !1065, file: !641, line: 560, type: !1079, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !379, declaration: !1083, retainedNodes: !1084)
!1079 = !DISubroutineType(types: !1080)
!1080 = !{!1081, !1082}
!1081 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u64", baseType: !360, size: 32, align: 32, dwarfAddressSpace: 0)
!1082 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<u64>", baseType: !1065, size: 32, align: 32, dwarfAddressSpace: 0)
!1083 = !DISubprogram(name: "as_mut_ptr<u64>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17he45af68aefaeac8fE", scope: !1065, file: !641, line: 560, type: !1079, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !379)
!1084 = !{!1077}
!1085 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !1086)
!1086 = distinct !DILocation(line: 1284, column: 70, scope: !1064)
!1087 = !DILocation(line: 1284, column: 66, scope: !1064)
!1088 = !DILocation(line: 1284, column: 15, scope: !1064)
!1089 = !DILocation(line: 1284, column: 9, scope: !1073)
!1090 = !DILocation(line: 1285, column: 5, scope: !1073)
!1091 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !1092)
!1092 = distinct !DILocation(line: 1286, column: 37, scope: !1073)
!1093 = !DILocation(line: 1286, column: 33, scope: !1073)
!1094 = !DILocation(line: 1286, column: 17, scope: !1073)
!1095 = !DILocation(line: 1286, column: 14, scope: !1073)
!1096 = !DILocation(line: 1286, column: 77, scope: !1073)
!1097 = !DILocation(line: 1287, column: 24, scope: !1073)
!1098 = !DILocation(line: 1287, column: 14, scope: !1073)
!1099 = !DILocation(line: 1287, column: 35, scope: !1073)
!1100 = !DILocation(line: 1289, column: 2, scope: !1044)
!1101 = distinct !DISubprogram(name: "fd_fdstat_get", linkageName: "_ZN4wasi13lib_generated13fd_fdstat_get17h5918041547ca7d50E", scope: !351, file: !588, line: 1378, type: !1102, scopeLine: 1378, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1118)
!1102 = !DISubroutineType(types: !1103)
!1103 = !{!1104, !44}
!1104 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<wasi::lib_generated::Fdstat, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 256, align: 64, flags: DIFlagPublic, elements: !1105, templateParams: !13, identifier: "6fa9b629d93981e24e38f1e1a8c384c")
!1105 = !{!1106}
!1106 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1104, file: !2, size: 256, align: 64, elements: !1107, templateParams: !13, identifier: "8ba42c464d8c21346df9897f467f9ad2", discriminator: !1117)
!1107 = !{!1108, !1113}
!1108 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1106, file: !2, baseType: !1109, size: 256, align: 64, extraData: i16 0)
!1109 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1104, file: !2, size: 256, align: 64, flags: DIFlagPublic, elements: !1110, templateParams: !1112, identifier: "5e6e0d3d28d91cc9e33a97a6dd86dea3")
!1110 = !{!1111}
!1111 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1109, file: !2, baseType: !350, size: 192, align: 64, offset: 64, flags: DIFlagPublic)
!1112 = !{!366, !601}
!1113 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1106, file: !2, baseType: !1114, size: 256, align: 64, extraData: i16 1)
!1114 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1104, file: !2, size: 256, align: 64, flags: DIFlagPublic, elements: !1115, templateParams: !1112, identifier: "d8ad145779cf570513202d27185494f8")
!1115 = !{!1116}
!1116 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1114, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1117 = !DIDerivedType(tag: DW_TAG_member, scope: !1104, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!1118 = !{!1119, !1120, !1129}
!1119 = !DILocalVariable(name: "fd", arg: 1, scope: !1101, file: !588, line: 1378, type: !44)
!1120 = !DILocalVariable(name: "rp0", scope: !1121, file: !588, line: 1379, type: !1122, align: 64)
!1121 = distinct !DILexicalBlock(scope: !1101, file: !588, line: 1379, column: 5)
!1122 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<wasi::lib_generated::Fdstat>", scope: !619, file: !2, size: 192, align: 64, elements: !1123, templateParams: !365, identifier: "6ffa063cd76182ea76c2759c77ba7fbf")
!1123 = !{!1124, !1125}
!1124 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !1122, file: !2, baseType: !7, align: 8)
!1125 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1122, file: !2, baseType: !1126, size: 192, align: 64)
!1126 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<wasi::lib_generated::Fdstat>", scope: !625, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1127, templateParams: !365, identifier: "1e32d9f8c214dcebdf5aa2b8df8e945b")
!1127 = !{!1128}
!1128 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1126, file: !2, baseType: !350, size: 192, align: 64, flags: DIFlagPrivate)
!1129 = !DILocalVariable(name: "ret", scope: !1130, file: !588, line: 1380, type: !631, align: 32)
!1130 = distinct !DILexicalBlock(scope: !1121, file: !588, line: 1380, column: 5)
!1131 = !DILocation(line: 1378, column: 29, scope: !1101)
!1132 = !DILocation(line: 1379, column: 9, scope: !1121)
!1133 = !DILocalVariable(name: "self", arg: 1, scope: !1134, file: !641, line: 560, type: !1138)
!1134 = distinct !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Fdstat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h8846d209218cbc74E", scope: !1122, file: !641, line: 560, type: !1135, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !365, declaration: !1139, retainedNodes: !1140)
!1135 = !DISubroutineType(types: !1136)
!1136 = !{!1137, !1138}
!1137 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut wasi::lib_generated::Fdstat", baseType: !350, size: 32, align: 32, dwarfAddressSpace: 0)
!1138 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<wasi::lib_generated::Fdstat>", baseType: !1122, size: 32, align: 32, dwarfAddressSpace: 0)
!1139 = !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Fdstat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h8846d209218cbc74E", scope: !1122, file: !641, line: 560, type: !1135, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !365)
!1140 = !{!1133}
!1141 = !DILocation(line: 560, column: 29, scope: !1134, inlinedAt: !1142)
!1142 = distinct !DILocation(line: 1380, column: 68, scope: !1121)
!1143 = !DILocation(line: 1380, column: 64, scope: !1121)
!1144 = !DILocation(line: 1380, column: 15, scope: !1121)
!1145 = !DILocation(line: 1380, column: 9, scope: !1130)
!1146 = !DILocation(line: 1381, column: 5, scope: !1130)
!1147 = !DILocation(line: 560, column: 29, scope: !1134, inlinedAt: !1148)
!1148 = distinct !DILocation(line: 1382, column: 37, scope: !1130)
!1149 = !DILocation(line: 1382, column: 33, scope: !1130)
!1150 = !DILocation(line: 1382, column: 17, scope: !1130)
!1151 = !DILocation(line: 1382, column: 14, scope: !1130)
!1152 = !DILocation(line: 1382, column: 74, scope: !1130)
!1153 = !DILocation(line: 1383, column: 24, scope: !1130)
!1154 = !DILocation(line: 1383, column: 14, scope: !1130)
!1155 = !DILocation(line: 1383, column: 35, scope: !1130)
!1156 = !DILocation(line: 1385, column: 2, scope: !1101)
!1157 = distinct !DISubprogram(name: "path_readlink", linkageName: "_ZN4wasi13lib_generated13path_readlink17hec5e259a0bd2226dE", scope: !351, file: !588, line: 1879, type: !1158, scopeLine: 1879, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1160)
!1158 = !DISubroutineType(types: !1159)
!1159 = !{!591, !44, !22, !610, !9}
!1160 = !{!1161, !1162, !1163, !1164, !1165, !1167}
!1161 = !DILocalVariable(name: "fd", arg: 1, scope: !1157, file: !588, line: 1880, type: !44)
!1162 = !DILocalVariable(name: "path", arg: 2, scope: !1157, file: !588, line: 1881, type: !22)
!1163 = !DILocalVariable(name: "buf", arg: 3, scope: !1157, file: !588, line: 1882, type: !610)
!1164 = !DILocalVariable(name: "buf_len", arg: 4, scope: !1157, file: !588, line: 1883, type: !9)
!1165 = !DILocalVariable(name: "rp0", scope: !1166, file: !588, line: 1885, type: !618, align: 32)
!1166 = distinct !DILexicalBlock(scope: !1157, file: !588, line: 1885, column: 5)
!1167 = !DILocalVariable(name: "ret", scope: !1168, file: !588, line: 1886, type: !631, align: 32)
!1168 = distinct !DILexicalBlock(scope: !1166, file: !588, line: 1886, column: 5)
!1169 = !DILocation(line: 1880, column: 5, scope: !1157)
!1170 = !DILocation(line: 1881, column: 5, scope: !1157)
!1171 = !DILocation(line: 1882, column: 5, scope: !1157)
!1172 = !DILocation(line: 1883, column: 5, scope: !1157)
!1173 = !DILocation(line: 1885, column: 9, scope: !1166)
!1174 = !DILocation(line: 1885, column: 19, scope: !1157)
!1175 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1176)
!1176 = distinct !DILocation(line: 1888, column: 14, scope: !1166)
!1177 = !DILocation(line: 1888, column: 9, scope: !1166)
!1178 = !DILocation(line: 1889, column: 14, scope: !1166)
!1179 = !DILocation(line: 1890, column: 9, scope: !1166)
!1180 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1181)
!1181 = distinct !DILocation(line: 1892, column: 13, scope: !1166)
!1182 = !DILocation(line: 1892, column: 9, scope: !1166)
!1183 = !DILocation(line: 1886, column: 15, scope: !1166)
!1184 = !DILocation(line: 1886, column: 9, scope: !1168)
!1185 = !DILocation(line: 1894, column: 5, scope: !1168)
!1186 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1187)
!1187 = distinct !DILocation(line: 1895, column: 37, scope: !1168)
!1188 = !DILocation(line: 1895, column: 33, scope: !1168)
!1189 = !DILocation(line: 1895, column: 17, scope: !1168)
!1190 = !DILocation(line: 1895, column: 14, scope: !1168)
!1191 = !DILocation(line: 1895, column: 72, scope: !1168)
!1192 = !DILocation(line: 1896, column: 24, scope: !1168)
!1193 = !DILocation(line: 1896, column: 14, scope: !1168)
!1194 = !DILocation(line: 1896, column: 35, scope: !1168)
!1195 = !DILocation(line: 1898, column: 2, scope: !1157)
!1196 = distinct !DISubprogram(name: "sock_shutdown", linkageName: "_ZN4wasi13lib_generated13sock_shutdown17h725fe06176d0b15dE", scope: !351, file: !588, line: 2157, type: !1197, scopeLine: 2157, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1199)
!1197 = !DISubroutineType(types: !1198)
!1198 = !{!667, !44, !26}
!1199 = !{!1200, !1201, !1202}
!1200 = !DILocalVariable(name: "fd", arg: 1, scope: !1196, file: !588, line: 2157, type: !44)
!1201 = !DILocalVariable(name: "how", arg: 2, scope: !1196, file: !588, line: 2157, type: !26)
!1202 = !DILocalVariable(name: "ret", scope: !1203, file: !588, line: 2158, type: !631, align: 32)
!1203 = distinct !DILexicalBlock(scope: !1196, file: !588, line: 2158, column: 5)
!1204 = !DILocation(line: 2157, column: 29, scope: !1196)
!1205 = !DILocation(line: 2157, column: 37, scope: !1196)
!1206 = !DILocation(line: 2158, column: 64, scope: !1196)
!1207 = !DILocation(line: 2158, column: 15, scope: !1196)
!1208 = !DILocation(line: 2158, column: 9, scope: !1203)
!1209 = !DILocation(line: 2159, column: 5, scope: !1203)
!1210 = !DILocation(line: 2160, column: 14, scope: !1203)
!1211 = !DILocation(line: 2160, column: 19, scope: !1203)
!1212 = !DILocation(line: 2161, column: 24, scope: !1203)
!1213 = !DILocation(line: 2161, column: 14, scope: !1203)
!1214 = !DILocation(line: 2161, column: 35, scope: !1203)
!1215 = !DILocation(line: 2163, column: 2, scope: !1196)
!1216 = distinct !DISubprogram(name: "args_sizes_get", linkageName: "_ZN4wasi13lib_generated14args_sizes_get17h3d2ba99f1b1f0261E", scope: !351, file: !588, line: 1225, type: !1217, scopeLine: 1225, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1238)
!1217 = !DISubroutineType(types: !1218)
!1218 = !{!1219}
!1219 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(usize, usize), wasi::lib_generated::Errno>", scope: !60, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1220, templateParams: !13, identifier: "29f120649bb115c46f55dc31dffac7d6")
!1220 = !{!1221}
!1221 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1219, file: !2, size: 96, align: 32, elements: !1222, templateParams: !13, identifier: "43c621f832ce1a3cb4c45d806a2e6cdb", discriminator: !1237)
!1222 = !{!1223, !1233}
!1223 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1221, file: !2, baseType: !1224, size: 96, align: 32, extraData: i16 0)
!1224 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1219, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1225, templateParams: !1231, identifier: "91645e6193e8ade242b25037f156eef9")
!1225 = !{!1226}
!1226 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1224, file: !2, baseType: !1227, size: 64, align: 32, offset: 32, flags: DIFlagPublic)
!1227 = !DICompositeType(tag: DW_TAG_structure_type, name: "(usize, usize)", file: !2, size: 64, align: 32, elements: !1228, templateParams: !13, identifier: "2f134127956ac419dda29236a1891616")
!1228 = !{!1229, !1230}
!1229 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1227, file: !2, baseType: !9, size: 32, align: 32)
!1230 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1227, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!1231 = !{!1232, !601}
!1232 = !DITemplateTypeParameter(name: "T", type: !1227)
!1233 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1221, file: !2, baseType: !1234, size: 96, align: 32, extraData: i16 1)
!1234 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1219, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1235, templateParams: !1231, identifier: "d2626d8b8a9cf70a56f9e038e4985139")
!1235 = !{!1236}
!1236 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1234, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1237 = !DIDerivedType(tag: DW_TAG_member, scope: !1219, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!1238 = !{!1239, !1241, !1243}
!1239 = !DILocalVariable(name: "rp0", scope: !1240, file: !588, line: 1226, type: !618, align: 32)
!1240 = distinct !DILexicalBlock(scope: !1216, file: !588, line: 1226, column: 5)
!1241 = !DILocalVariable(name: "rp1", scope: !1242, file: !588, line: 1227, type: !618, align: 32)
!1242 = distinct !DILexicalBlock(scope: !1240, file: !588, line: 1227, column: 5)
!1243 = !DILocalVariable(name: "ret", scope: !1244, file: !588, line: 1228, type: !631, align: 32)
!1244 = distinct !DILexicalBlock(scope: !1242, file: !588, line: 1228, column: 5)
!1245 = !DILocation(line: 1226, column: 9, scope: !1240)
!1246 = !DILocation(line: 1227, column: 9, scope: !1242)
!1247 = !DILocation(line: 1226, column: 19, scope: !1216)
!1248 = !DILocation(line: 1227, column: 19, scope: !1240)
!1249 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1250)
!1250 = distinct !DILocation(line: 1229, column: 52, scope: !1242)
!1251 = !DILocation(line: 1229, column: 48, scope: !1242)
!1252 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1253)
!1253 = distinct !DILocation(line: 1229, column: 77, scope: !1242)
!1254 = !DILocation(line: 1229, column: 73, scope: !1242)
!1255 = !DILocation(line: 1229, column: 9, scope: !1242)
!1256 = !DILocation(line: 1228, column: 9, scope: !1244)
!1257 = !DILocation(line: 1230, column: 5, scope: !1244)
!1258 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1259)
!1259 = distinct !DILocation(line: 1232, column: 33, scope: !1244)
!1260 = !DILocation(line: 1232, column: 29, scope: !1244)
!1261 = !DILocation(line: 1232, column: 13, scope: !1244)
!1262 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1263)
!1263 = distinct !DILocation(line: 1233, column: 33, scope: !1244)
!1264 = !DILocation(line: 1233, column: 29, scope: !1244)
!1265 = !DILocation(line: 1233, column: 13, scope: !1244)
!1266 = !DILocation(line: 1231, column: 14, scope: !1244)
!1267 = !DILocation(line: 1234, column: 10, scope: !1244)
!1268 = !DILocation(line: 1235, column: 24, scope: !1244)
!1269 = !DILocation(line: 1235, column: 14, scope: !1244)
!1270 = !DILocation(line: 1235, column: 35, scope: !1244)
!1271 = !DILocation(line: 1237, column: 2, scope: !1216)
!1272 = distinct !DISubprogram(name: "clock_time_get", linkageName: "_ZN4wasi13lib_generated14clock_time_get17hf3af7872ec7af954E", scope: !351, file: !588, line: 1302, type: !1273, scopeLine: 1302, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1275)
!1273 = !DISubroutineType(types: !1274)
!1274 = !{!1047, !884, !360}
!1275 = !{!1276, !1277, !1278, !1280}
!1276 = !DILocalVariable(name: "id", arg: 1, scope: !1272, file: !588, line: 1302, type: !884)
!1277 = !DILocalVariable(name: "precision", arg: 2, scope: !1272, file: !588, line: 1302, type: !360)
!1278 = !DILocalVariable(name: "rp0", scope: !1279, file: !588, line: 1303, type: !1065, align: 64)
!1279 = distinct !DILexicalBlock(scope: !1272, file: !588, line: 1303, column: 5)
!1280 = !DILocalVariable(name: "ret", scope: !1281, file: !588, line: 1304, type: !631, align: 32)
!1281 = distinct !DILexicalBlock(scope: !1279, file: !588, line: 1304, column: 5)
!1282 = !DILocation(line: 1302, column: 30, scope: !1272)
!1283 = !DILocation(line: 1302, column: 43, scope: !1272)
!1284 = !DILocation(line: 1303, column: 9, scope: !1279)
!1285 = !DILocation(line: 1303, column: 19, scope: !1272)
!1286 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !1287)
!1287 = distinct !DILocation(line: 1307, column: 13, scope: !1279)
!1288 = !DILocation(line: 1307, column: 9, scope: !1279)
!1289 = !DILocation(line: 1304, column: 15, scope: !1279)
!1290 = !DILocation(line: 1304, column: 9, scope: !1281)
!1291 = !DILocation(line: 1309, column: 5, scope: !1281)
!1292 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !1293)
!1293 = distinct !DILocation(line: 1310, column: 37, scope: !1281)
!1294 = !DILocation(line: 1310, column: 33, scope: !1281)
!1295 = !DILocation(line: 1310, column: 17, scope: !1281)
!1296 = !DILocation(line: 1310, column: 14, scope: !1281)
!1297 = !DILocation(line: 1310, column: 77, scope: !1281)
!1298 = !DILocation(line: 1311, column: 24, scope: !1281)
!1299 = !DILocation(line: 1311, column: 14, scope: !1281)
!1300 = !DILocation(line: 1311, column: 35, scope: !1281)
!1301 = !DILocation(line: 1313, column: 2, scope: !1272)
!1302 = distinct !DISubprogram(name: "fd_prestat_get", linkageName: "_ZN4wasi13lib_generated14fd_prestat_get17hcb4b9e9d217424b3E", scope: !351, file: !588, line: 1508, type: !1303, scopeLine: 1508, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1319)
!1303 = !DISubroutineType(types: !1304)
!1304 = !{!1305, !44}
!1305 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<wasi::lib_generated::Prestat, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1306, templateParams: !13, identifier: "dcf43eb5b865025a9d47a9674b0c5b7f")
!1306 = !{!1307}
!1307 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1305, file: !2, size: 96, align: 32, elements: !1308, templateParams: !13, identifier: "9c7c1f8a49a5456844fcc65ed6b298e0", discriminator: !1318)
!1308 = !{!1309, !1314}
!1309 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1307, file: !2, baseType: !1310, size: 96, align: 32, extraData: i16 0)
!1310 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1305, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1311, templateParams: !1313, identifier: "ad65d6dd0b24fdffaea355aa6f31604f")
!1311 = !{!1312}
!1312 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1310, file: !2, baseType: !390, size: 64, align: 32, offset: 32, flags: DIFlagPublic)
!1313 = !{!404, !601}
!1314 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1307, file: !2, baseType: !1315, size: 96, align: 32, extraData: i16 1)
!1315 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1305, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !1316, templateParams: !1313, identifier: "e907670378272adb7a88337179434b35")
!1316 = !{!1317}
!1317 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1315, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1318 = !DIDerivedType(tag: DW_TAG_member, scope: !1305, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!1319 = !{!1320, !1321, !1330}
!1320 = !DILocalVariable(name: "fd", arg: 1, scope: !1302, file: !588, line: 1508, type: !44)
!1321 = !DILocalVariable(name: "rp0", scope: !1322, file: !588, line: 1509, type: !1323, align: 32)
!1322 = distinct !DILexicalBlock(scope: !1302, file: !588, line: 1509, column: 5)
!1323 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<wasi::lib_generated::Prestat>", scope: !619, file: !2, size: 64, align: 32, elements: !1324, templateParams: !403, identifier: "b26409a751ac81eb233772dd3d3b3461")
!1324 = !{!1325, !1326}
!1325 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !1323, file: !2, baseType: !7, align: 8)
!1326 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1323, file: !2, baseType: !1327, size: 64, align: 32)
!1327 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<wasi::lib_generated::Prestat>", scope: !625, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !1328, templateParams: !403, identifier: "2d5085488ed19d107f4b533a189af678")
!1328 = !{!1329}
!1329 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1327, file: !2, baseType: !390, size: 64, align: 32, flags: DIFlagPrivate)
!1330 = !DILocalVariable(name: "ret", scope: !1331, file: !588, line: 1510, type: !631, align: 32)
!1331 = distinct !DILexicalBlock(scope: !1322, file: !588, line: 1510, column: 5)
!1332 = !DILocation(line: 1508, column: 30, scope: !1302)
!1333 = !DILocation(line: 1509, column: 9, scope: !1322)
!1334 = !DILocalVariable(name: "self", arg: 1, scope: !1335, file: !641, line: 560, type: !1339)
!1335 = distinct !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Prestat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h7a19e13ec52bb68cE", scope: !1323, file: !641, line: 560, type: !1336, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !403, declaration: !1340, retainedNodes: !1341)
!1336 = !DISubroutineType(types: !1337)
!1337 = !{!1338, !1339}
!1338 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut wasi::lib_generated::Prestat", baseType: !390, size: 32, align: 32, dwarfAddressSpace: 0)
!1339 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<wasi::lib_generated::Prestat>", baseType: !1323, size: 32, align: 32, dwarfAddressSpace: 0)
!1340 = !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Prestat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h7a19e13ec52bb68cE", scope: !1323, file: !641, line: 560, type: !1336, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !403)
!1341 = !{!1334}
!1342 = !DILocation(line: 560, column: 29, scope: !1335, inlinedAt: !1343)
!1343 = distinct !DILocation(line: 1510, column: 69, scope: !1322)
!1344 = !DILocation(line: 1510, column: 65, scope: !1322)
!1345 = !DILocation(line: 1510, column: 15, scope: !1322)
!1346 = !DILocation(line: 1510, column: 9, scope: !1331)
!1347 = !DILocation(line: 1511, column: 5, scope: !1331)
!1348 = !DILocation(line: 560, column: 29, scope: !1335, inlinedAt: !1349)
!1349 = distinct !DILocation(line: 1512, column: 37, scope: !1331)
!1350 = !DILocation(line: 1512, column: 33, scope: !1331)
!1351 = !DILocation(line: 1512, column: 17, scope: !1331)
!1352 = !DILocation(line: 1512, column: 14, scope: !1331)
!1353 = !DILocation(line: 1512, column: 75, scope: !1331)
!1354 = !DILocation(line: 1513, column: 24, scope: !1331)
!1355 = !DILocation(line: 1513, column: 14, scope: !1331)
!1356 = !DILocation(line: 1513, column: 35, scope: !1331)
!1357 = !DILocation(line: 1515, column: 2, scope: !1302)
!1358 = distinct !DISubprogram(name: "fd_filestat_get", linkageName: "_ZN4wasi13lib_generated15fd_filestat_get17h0bad099bde016b06E", scope: !351, file: !588, line: 1428, type: !1359, scopeLine: 1428, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1375)
!1359 = !DISubroutineType(types: !1360)
!1360 = !{!1361, !44}
!1361 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<wasi::lib_generated::Filestat, wasi::lib_generated::Errno>", scope: !60, file: !2, size: 576, align: 64, flags: DIFlagPublic, elements: !1362, templateParams: !13, identifier: "32497f5793bdeca857258f6f86f25829")
!1362 = !{!1363}
!1363 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1361, file: !2, size: 576, align: 64, elements: !1364, templateParams: !13, identifier: "a2d26414504d73d94f62bf52149eb706", discriminator: !1374)
!1364 = !{!1365, !1370}
!1365 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1363, file: !2, baseType: !1366, size: 576, align: 64, extraData: i16 0)
!1366 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1361, file: !2, size: 576, align: 64, flags: DIFlagPublic, elements: !1367, templateParams: !1369, identifier: "754e142e351895ed85865057b7ccb5d2")
!1367 = !{!1368}
!1368 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1366, file: !2, baseType: !414, size: 512, align: 64, offset: 64, flags: DIFlagPublic)
!1369 = !{!428, !601}
!1370 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1363, file: !2, baseType: !1371, size: 576, align: 64, extraData: i16 1)
!1371 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1361, file: !2, size: 576, align: 64, flags: DIFlagPublic, elements: !1372, templateParams: !1369, identifier: "eb4a091776da88b96d79b3958ec1abe9")
!1372 = !{!1373}
!1373 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1371, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!1374 = !DIDerivedType(tag: DW_TAG_member, scope: !1361, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!1375 = !{!1376, !1377, !1386}
!1376 = !DILocalVariable(name: "fd", arg: 1, scope: !1358, file: !588, line: 1428, type: !44)
!1377 = !DILocalVariable(name: "rp0", scope: !1378, file: !588, line: 1429, type: !1379, align: 64)
!1378 = distinct !DILexicalBlock(scope: !1358, file: !588, line: 1429, column: 5)
!1379 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<wasi::lib_generated::Filestat>", scope: !619, file: !2, size: 512, align: 64, elements: !1380, templateParams: !427, identifier: "786388a3d8bfb071591e7f1222580d36")
!1380 = !{!1381, !1382}
!1381 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !1379, file: !2, baseType: !7, align: 8)
!1382 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1379, file: !2, baseType: !1383, size: 512, align: 64)
!1383 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<wasi::lib_generated::Filestat>", scope: !625, file: !2, size: 512, align: 64, flags: DIFlagPublic, elements: !1384, templateParams: !427, identifier: "aae9a7cfb1679646f1900240bb4c956")
!1384 = !{!1385}
!1385 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !1383, file: !2, baseType: !414, size: 512, align: 64, flags: DIFlagPrivate)
!1386 = !DILocalVariable(name: "ret", scope: !1387, file: !588, line: 1430, type: !631, align: 32)
!1387 = distinct !DILexicalBlock(scope: !1378, file: !588, line: 1430, column: 5)
!1388 = !DILocation(line: 1428, column: 31, scope: !1358)
!1389 = !DILocation(line: 1429, column: 9, scope: !1378)
!1390 = !DILocalVariable(name: "self", arg: 1, scope: !1391, file: !641, line: 560, type: !1395)
!1391 = distinct !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Filestat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hcf35fe8490e349e9E", scope: !1379, file: !641, line: 560, type: !1392, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !427, declaration: !1396, retainedNodes: !1397)
!1392 = !DISubroutineType(types: !1393)
!1393 = !{!1394, !1395}
!1394 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut wasi::lib_generated::Filestat", baseType: !414, size: 32, align: 32, dwarfAddressSpace: 0)
!1395 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<wasi::lib_generated::Filestat>", baseType: !1379, size: 32, align: 32, dwarfAddressSpace: 0)
!1396 = !DISubprogram(name: "as_mut_ptr<wasi::lib_generated::Filestat>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hcf35fe8490e349e9E", scope: !1379, file: !641, line: 560, type: !1392, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !427)
!1397 = !{!1390}
!1398 = !DILocation(line: 560, column: 29, scope: !1391, inlinedAt: !1399)
!1399 = distinct !DILocation(line: 1430, column: 70, scope: !1378)
!1400 = !DILocation(line: 1430, column: 66, scope: !1378)
!1401 = !DILocation(line: 1430, column: 15, scope: !1378)
!1402 = !DILocation(line: 1430, column: 9, scope: !1387)
!1403 = !DILocation(line: 1431, column: 5, scope: !1387)
!1404 = !DILocation(line: 560, column: 29, scope: !1391, inlinedAt: !1405)
!1405 = distinct !DILocation(line: 1432, column: 37, scope: !1387)
!1406 = !DILocation(line: 1432, column: 33, scope: !1387)
!1407 = !DILocation(line: 1432, column: 17, scope: !1387)
!1408 = !DILocation(line: 1432, column: 14, scope: !1387)
!1409 = !DILocation(line: 1432, column: 76, scope: !1387)
!1410 = !DILocation(line: 1433, column: 24, scope: !1387)
!1411 = !DILocation(line: 1433, column: 14, scope: !1387)
!1412 = !DILocation(line: 1433, column: 35, scope: !1387)
!1413 = !DILocation(line: 1435, column: 2, scope: !1358)
!1414 = distinct !DISubprogram(name: "path_unlink_file", linkageName: "_ZN4wasi13lib_generated16path_unlink_file17hfd3c14cbf52140efE", scope: !351, file: !588, line: 1970, type: !1415, scopeLine: 1970, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1417)
!1415 = !DISubroutineType(types: !1416)
!1416 = !{!667, !44, !22}
!1417 = !{!1418, !1419, !1420}
!1418 = !DILocalVariable(name: "fd", arg: 1, scope: !1414, file: !588, line: 1970, type: !44)
!1419 = !DILocalVariable(name: "path", arg: 2, scope: !1414, file: !588, line: 1970, type: !22)
!1420 = !DILocalVariable(name: "ret", scope: !1421, file: !588, line: 1971, type: !631, align: 32)
!1421 = distinct !DILexicalBlock(scope: !1414, file: !588, line: 1971, column: 5)
!1422 = !DILocation(line: 1970, column: 32, scope: !1414)
!1423 = !DILocation(line: 1970, column: 40, scope: !1414)
!1424 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1425)
!1425 = distinct !DILocation(line: 1973, column: 14, scope: !1414)
!1426 = !DILocation(line: 1973, column: 9, scope: !1414)
!1427 = !DILocation(line: 1974, column: 14, scope: !1414)
!1428 = !DILocation(line: 1971, column: 15, scope: !1414)
!1429 = !DILocation(line: 1971, column: 9, scope: !1421)
!1430 = !DILocation(line: 1976, column: 5, scope: !1421)
!1431 = !DILocation(line: 1977, column: 14, scope: !1421)
!1432 = !DILocation(line: 1977, column: 19, scope: !1421)
!1433 = !DILocation(line: 1978, column: 24, scope: !1421)
!1434 = !DILocation(line: 1978, column: 14, scope: !1421)
!1435 = !DILocation(line: 1978, column: 35, scope: !1421)
!1436 = !DILocation(line: 1980, column: 2, scope: !1414)
!1437 = distinct !DISubprogram(name: "environ_sizes_get", linkageName: "_ZN4wasi13lib_generated17environ_sizes_get17haf8697964351a6a8E", scope: !351, file: !588, line: 1256, type: !1217, scopeLine: 1256, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1438)
!1438 = !{!1439, !1441, !1443}
!1439 = !DILocalVariable(name: "rp0", scope: !1440, file: !588, line: 1257, type: !618, align: 32)
!1440 = distinct !DILexicalBlock(scope: !1437, file: !588, line: 1257, column: 5)
!1441 = !DILocalVariable(name: "rp1", scope: !1442, file: !588, line: 1258, type: !618, align: 32)
!1442 = distinct !DILexicalBlock(scope: !1440, file: !588, line: 1258, column: 5)
!1443 = !DILocalVariable(name: "ret", scope: !1444, file: !588, line: 1259, type: !631, align: 32)
!1444 = distinct !DILexicalBlock(scope: !1442, file: !588, line: 1259, column: 5)
!1445 = !DILocation(line: 1257, column: 9, scope: !1440)
!1446 = !DILocation(line: 1258, column: 9, scope: !1442)
!1447 = !DILocation(line: 1257, column: 19, scope: !1437)
!1448 = !DILocation(line: 1258, column: 19, scope: !1440)
!1449 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1450)
!1450 = distinct !DILocation(line: 1260, column: 55, scope: !1442)
!1451 = !DILocation(line: 1260, column: 51, scope: !1442)
!1452 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1453)
!1453 = distinct !DILocation(line: 1260, column: 80, scope: !1442)
!1454 = !DILocation(line: 1260, column: 76, scope: !1442)
!1455 = !DILocation(line: 1260, column: 9, scope: !1442)
!1456 = !DILocation(line: 1259, column: 9, scope: !1444)
!1457 = !DILocation(line: 1261, column: 5, scope: !1444)
!1458 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1459)
!1459 = distinct !DILocation(line: 1263, column: 33, scope: !1444)
!1460 = !DILocation(line: 1263, column: 29, scope: !1444)
!1461 = !DILocation(line: 1263, column: 13, scope: !1444)
!1462 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !1463)
!1463 = distinct !DILocation(line: 1264, column: 33, scope: !1444)
!1464 = !DILocation(line: 1264, column: 29, scope: !1444)
!1465 = !DILocation(line: 1264, column: 13, scope: !1444)
!1466 = !DILocation(line: 1262, column: 14, scope: !1444)
!1467 = !DILocation(line: 1265, column: 10, scope: !1444)
!1468 = !DILocation(line: 1266, column: 24, scope: !1444)
!1469 = !DILocation(line: 1266, column: 14, scope: !1444)
!1470 = !DILocation(line: 1266, column: 35, scope: !1444)
!1471 = !DILocation(line: 1268, column: 2, scope: !1437)
!1472 = distinct !DISubprogram(name: "path_filestat_get", linkageName: "_ZN4wasi13lib_generated17path_filestat_get17h36e722cead09cc29E", scope: !351, file: !588, line: 1737, type: !1473, scopeLine: 1737, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1475)
!1473 = !DISubroutineType(types: !1474)
!1474 = !{!1361, !44, !44, !22}
!1475 = !{!1476, !1477, !1478, !1479, !1481}
!1476 = !DILocalVariable(name: "fd", arg: 1, scope: !1472, file: !588, line: 1737, type: !44)
!1477 = !DILocalVariable(name: "flags", arg: 2, scope: !1472, file: !588, line: 1737, type: !44)
!1478 = !DILocalVariable(name: "path", arg: 3, scope: !1472, file: !588, line: 1737, type: !22)
!1479 = !DILocalVariable(name: "rp0", scope: !1480, file: !588, line: 1738, type: !1379, align: 64)
!1480 = distinct !DILexicalBlock(scope: !1472, file: !588, line: 1738, column: 5)
!1481 = !DILocalVariable(name: "ret", scope: !1482, file: !588, line: 1739, type: !631, align: 32)
!1482 = distinct !DILexicalBlock(scope: !1480, file: !588, line: 1739, column: 5)
!1483 = !DILocation(line: 1737, column: 33, scope: !1472)
!1484 = !DILocation(line: 1737, column: 41, scope: !1472)
!1485 = !DILocation(line: 1737, column: 61, scope: !1472)
!1486 = !DILocation(line: 1738, column: 9, scope: !1480)
!1487 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1488)
!1488 = distinct !DILocation(line: 1742, column: 14, scope: !1480)
!1489 = !DILocation(line: 1742, column: 9, scope: !1480)
!1490 = !DILocation(line: 1743, column: 14, scope: !1480)
!1491 = !DILocation(line: 560, column: 29, scope: !1391, inlinedAt: !1492)
!1492 = distinct !DILocation(line: 1744, column: 13, scope: !1480)
!1493 = !DILocation(line: 1744, column: 9, scope: !1480)
!1494 = !DILocation(line: 1739, column: 15, scope: !1480)
!1495 = !DILocation(line: 1739, column: 9, scope: !1482)
!1496 = !DILocation(line: 1746, column: 5, scope: !1482)
!1497 = !DILocation(line: 560, column: 29, scope: !1391, inlinedAt: !1498)
!1498 = distinct !DILocation(line: 1747, column: 37, scope: !1482)
!1499 = !DILocation(line: 1747, column: 33, scope: !1482)
!1500 = !DILocation(line: 1747, column: 17, scope: !1482)
!1501 = !DILocation(line: 1747, column: 14, scope: !1482)
!1502 = !DILocation(line: 1747, column: 76, scope: !1482)
!1503 = !DILocation(line: 1748, column: 24, scope: !1482)
!1504 = !DILocation(line: 1748, column: 14, scope: !1482)
!1505 = !DILocation(line: 1748, column: 35, scope: !1482)
!1506 = !DILocation(line: 1750, column: 2, scope: !1472)
!1507 = distinct !DISubprogram(name: "fd_fdstat_set_flags", linkageName: "_ZN4wasi13lib_generated19fd_fdstat_set_flags17h61a62bf5e192b128E", scope: !351, file: !588, line: 1393, type: !1508, scopeLine: 1393, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1510)
!1508 = !DISubroutineType(types: !1509)
!1509 = !{!667, !44, !12}
!1510 = !{!1511, !1512, !1513}
!1511 = !DILocalVariable(name: "fd", arg: 1, scope: !1507, file: !588, line: 1393, type: !44)
!1512 = !DILocalVariable(name: "flags", arg: 2, scope: !1507, file: !588, line: 1393, type: !12)
!1513 = !DILocalVariable(name: "ret", scope: !1514, file: !588, line: 1394, type: !631, align: 32)
!1514 = distinct !DILexicalBlock(scope: !1507, file: !588, line: 1394, column: 5)
!1515 = !DILocation(line: 1393, column: 35, scope: !1507)
!1516 = !DILocation(line: 1393, column: 43, scope: !1507)
!1517 = !DILocation(line: 1394, column: 70, scope: !1507)
!1518 = !DILocation(line: 1394, column: 15, scope: !1507)
!1519 = !DILocation(line: 1394, column: 9, scope: !1514)
!1520 = !DILocation(line: 1395, column: 5, scope: !1514)
!1521 = !DILocation(line: 1396, column: 14, scope: !1514)
!1522 = !DILocation(line: 1396, column: 19, scope: !1514)
!1523 = !DILocation(line: 1397, column: 24, scope: !1514)
!1524 = !DILocation(line: 1397, column: 14, scope: !1514)
!1525 = !DILocation(line: 1397, column: 35, scope: !1514)
!1526 = !DILocation(line: 1399, column: 2, scope: !1507)
!1527 = distinct !DISubprogram(name: "fd_prestat_dir_name", linkageName: "_ZN4wasi13lib_generated19fd_prestat_dir_name17h86844aaa09e395d0E", scope: !351, file: !588, line: 1522, type: !1528, scopeLine: 1522, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1530)
!1528 = !DISubroutineType(types: !1529)
!1529 = !{!667, !44, !610, !9}
!1530 = !{!1531, !1532, !1533, !1534}
!1531 = !DILocalVariable(name: "fd", arg: 1, scope: !1527, file: !588, line: 1522, type: !44)
!1532 = !DILocalVariable(name: "path", arg: 2, scope: !1527, file: !588, line: 1522, type: !610)
!1533 = !DILocalVariable(name: "path_len", arg: 3, scope: !1527, file: !588, line: 1522, type: !9)
!1534 = !DILocalVariable(name: "ret", scope: !1535, file: !588, line: 1523, type: !631, align: 32)
!1535 = distinct !DILexicalBlock(scope: !1527, file: !588, line: 1523, column: 5)
!1536 = !DILocation(line: 1522, column: 35, scope: !1527)
!1537 = !DILocation(line: 1522, column: 43, scope: !1527)
!1538 = !DILocation(line: 1522, column: 58, scope: !1527)
!1539 = !DILocation(line: 1523, column: 70, scope: !1527)
!1540 = !DILocation(line: 1523, column: 15, scope: !1527)
!1541 = !DILocation(line: 1523, column: 9, scope: !1535)
!1542 = !DILocation(line: 1524, column: 5, scope: !1535)
!1543 = !DILocation(line: 1525, column: 14, scope: !1535)
!1544 = !DILocation(line: 1525, column: 19, scope: !1535)
!1545 = !DILocation(line: 1526, column: 24, scope: !1535)
!1546 = !DILocation(line: 1526, column: 14, scope: !1535)
!1547 = !DILocation(line: 1526, column: 35, scope: !1535)
!1548 = !DILocation(line: 1528, column: 2, scope: !1527)
!1549 = distinct !DISubprogram(name: "fd_fdstat_set_rights", linkageName: "_ZN4wasi13lib_generated20fd_fdstat_set_rights17h3601f3e6d2a09b9bE", scope: !351, file: !588, line: 1407, type: !773, scopeLine: 1407, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1550)
!1550 = !{!1551, !1552, !1553, !1554}
!1551 = !DILocalVariable(name: "fd", arg: 1, scope: !1549, file: !588, line: 1408, type: !44)
!1552 = !DILocalVariable(name: "fs_rights_base", arg: 2, scope: !1549, file: !588, line: 1409, type: !360)
!1553 = !DILocalVariable(name: "fs_rights_inheriting", arg: 3, scope: !1549, file: !588, line: 1410, type: !360)
!1554 = !DILocalVariable(name: "ret", scope: !1555, file: !588, line: 1412, type: !631, align: 32)
!1555 = distinct !DILexicalBlock(scope: !1549, file: !588, line: 1412, column: 5)
!1556 = !DILocation(line: 1408, column: 5, scope: !1549)
!1557 = !DILocation(line: 1409, column: 5, scope: !1549)
!1558 = !DILocation(line: 1410, column: 5, scope: !1549)
!1559 = !DILocation(line: 1412, column: 15, scope: !1549)
!1560 = !DILocation(line: 1412, column: 9, scope: !1555)
!1561 = !DILocation(line: 1417, column: 5, scope: !1555)
!1562 = !DILocation(line: 1418, column: 14, scope: !1555)
!1563 = !DILocation(line: 1418, column: 19, scope: !1555)
!1564 = !DILocation(line: 1419, column: 24, scope: !1555)
!1565 = !DILocation(line: 1419, column: 14, scope: !1555)
!1566 = !DILocation(line: 1419, column: 35, scope: !1555)
!1567 = !DILocation(line: 1421, column: 2, scope: !1549)
!1568 = distinct !DISubprogram(name: "fd_filestat_set_size", linkageName: "_ZN4wasi13lib_generated20fd_filestat_set_size17hbd7e7495799fbe2fE", scope: !351, file: !588, line: 1443, type: !1569, scopeLine: 1443, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1571)
!1569 = !DISubroutineType(types: !1570)
!1570 = !{!667, !44, !360}
!1571 = !{!1572, !1573, !1574}
!1572 = !DILocalVariable(name: "fd", arg: 1, scope: !1568, file: !588, line: 1443, type: !44)
!1573 = !DILocalVariable(name: "size", arg: 2, scope: !1568, file: !588, line: 1443, type: !360)
!1574 = !DILocalVariable(name: "ret", scope: !1575, file: !588, line: 1444, type: !631, align: 32)
!1575 = distinct !DILexicalBlock(scope: !1568, file: !588, line: 1444, column: 5)
!1576 = !DILocation(line: 1443, column: 36, scope: !1568)
!1577 = !DILocation(line: 1443, column: 44, scope: !1568)
!1578 = !DILocation(line: 1444, column: 15, scope: !1568)
!1579 = !DILocation(line: 1444, column: 9, scope: !1575)
!1580 = !DILocation(line: 1445, column: 5, scope: !1575)
!1581 = !DILocation(line: 1446, column: 14, scope: !1575)
!1582 = !DILocation(line: 1446, column: 19, scope: !1575)
!1583 = !DILocation(line: 1447, column: 24, scope: !1575)
!1584 = !DILocation(line: 1447, column: 14, scope: !1575)
!1585 = !DILocation(line: 1447, column: 35, scope: !1575)
!1586 = !DILocation(line: 1449, column: 2, scope: !1568)
!1587 = distinct !DISubprogram(name: "fd_filestat_set_times", linkageName: "_ZN4wasi13lib_generated21fd_filestat_set_times17hb6ca76dc5c47ded6E", scope: !351, file: !588, line: 1459, type: !1588, scopeLine: 1459, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1590)
!1588 = !DISubroutineType(types: !1589)
!1589 = !{!667, !44, !360, !360, !12}
!1590 = !{!1591, !1592, !1593, !1594, !1595}
!1591 = !DILocalVariable(name: "fd", arg: 1, scope: !1587, file: !588, line: 1460, type: !44)
!1592 = !DILocalVariable(name: "atim", arg: 2, scope: !1587, file: !588, line: 1461, type: !360)
!1593 = !DILocalVariable(name: "mtim", arg: 3, scope: !1587, file: !588, line: 1462, type: !360)
!1594 = !DILocalVariable(name: "fst_flags", arg: 4, scope: !1587, file: !588, line: 1463, type: !12)
!1595 = !DILocalVariable(name: "ret", scope: !1596, file: !588, line: 1465, type: !631, align: 32)
!1596 = distinct !DILexicalBlock(scope: !1587, file: !588, line: 1465, column: 5)
!1597 = !DILocation(line: 1460, column: 5, scope: !1587)
!1598 = !DILocation(line: 1461, column: 5, scope: !1587)
!1599 = !DILocation(line: 1462, column: 5, scope: !1587)
!1600 = !DILocation(line: 1463, column: 5, scope: !1587)
!1601 = !DILocation(line: 1469, column: 9, scope: !1587)
!1602 = !DILocation(line: 1465, column: 15, scope: !1587)
!1603 = !DILocation(line: 1465, column: 9, scope: !1596)
!1604 = !DILocation(line: 1471, column: 5, scope: !1596)
!1605 = !DILocation(line: 1472, column: 14, scope: !1596)
!1606 = !DILocation(line: 1472, column: 19, scope: !1596)
!1607 = !DILocation(line: 1473, column: 24, scope: !1596)
!1608 = !DILocation(line: 1473, column: 14, scope: !1596)
!1609 = !DILocation(line: 1473, column: 35, scope: !1596)
!1610 = !DILocation(line: 1475, column: 2, scope: !1587)
!1611 = distinct !DISubprogram(name: "path_create_directory", linkageName: "_ZN4wasi13lib_generated21path_create_directory17h51aa6b558786c7beE", scope: !351, file: !588, line: 1714, type: !1415, scopeLine: 1714, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1612)
!1612 = !{!1613, !1614, !1615}
!1613 = !DILocalVariable(name: "fd", arg: 1, scope: !1611, file: !588, line: 1714, type: !44)
!1614 = !DILocalVariable(name: "path", arg: 2, scope: !1611, file: !588, line: 1714, type: !22)
!1615 = !DILocalVariable(name: "ret", scope: !1616, file: !588, line: 1715, type: !631, align: 32)
!1616 = distinct !DILexicalBlock(scope: !1611, file: !588, line: 1715, column: 5)
!1617 = !DILocation(line: 1714, column: 37, scope: !1611)
!1618 = !DILocation(line: 1714, column: 45, scope: !1611)
!1619 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1620)
!1620 = distinct !DILocation(line: 1717, column: 14, scope: !1611)
!1621 = !DILocation(line: 1717, column: 9, scope: !1611)
!1622 = !DILocation(line: 1718, column: 14, scope: !1611)
!1623 = !DILocation(line: 1715, column: 15, scope: !1611)
!1624 = !DILocation(line: 1715, column: 9, scope: !1616)
!1625 = !DILocation(line: 1720, column: 5, scope: !1616)
!1626 = !DILocation(line: 1721, column: 14, scope: !1616)
!1627 = !DILocation(line: 1721, column: 19, scope: !1616)
!1628 = !DILocation(line: 1722, column: 24, scope: !1616)
!1629 = !DILocation(line: 1722, column: 14, scope: !1616)
!1630 = !DILocation(line: 1722, column: 35, scope: !1616)
!1631 = !DILocation(line: 1724, column: 2, scope: !1611)
!1632 = distinct !DISubprogram(name: "path_remove_directory", linkageName: "_ZN4wasi13lib_generated21path_remove_directory17h454076b0e3db5ad7E", scope: !351, file: !588, line: 1907, type: !1415, scopeLine: 1907, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1633)
!1633 = !{!1634, !1635, !1636}
!1634 = !DILocalVariable(name: "fd", arg: 1, scope: !1632, file: !588, line: 1907, type: !44)
!1635 = !DILocalVariable(name: "path", arg: 2, scope: !1632, file: !588, line: 1907, type: !22)
!1636 = !DILocalVariable(name: "ret", scope: !1637, file: !588, line: 1908, type: !631, align: 32)
!1637 = distinct !DILexicalBlock(scope: !1632, file: !588, line: 1908, column: 5)
!1638 = !DILocation(line: 1907, column: 37, scope: !1632)
!1639 = !DILocation(line: 1907, column: 45, scope: !1632)
!1640 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1641)
!1641 = distinct !DILocation(line: 1910, column: 14, scope: !1632)
!1642 = !DILocation(line: 1910, column: 9, scope: !1632)
!1643 = !DILocation(line: 1911, column: 14, scope: !1632)
!1644 = !DILocation(line: 1908, column: 15, scope: !1632)
!1645 = !DILocation(line: 1908, column: 9, scope: !1637)
!1646 = !DILocation(line: 1913, column: 5, scope: !1637)
!1647 = !DILocation(line: 1914, column: 14, scope: !1637)
!1648 = !DILocation(line: 1914, column: 19, scope: !1637)
!1649 = !DILocation(line: 1915, column: 24, scope: !1637)
!1650 = !DILocation(line: 1915, column: 14, scope: !1637)
!1651 = !DILocation(line: 1915, column: 35, scope: !1637)
!1652 = !DILocation(line: 1917, column: 2, scope: !1632)
!1653 = distinct !DISubprogram(name: "path_filestat_set_times", linkageName: "_ZN4wasi13lib_generated23path_filestat_set_times17h14a5913f2496ec36E", scope: !351, file: !588, line: 1762, type: !1654, scopeLine: 1762, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !1656)
!1654 = !DISubroutineType(types: !1655)
!1655 = !{!667, !44, !44, !22, !360, !360, !12}
!1656 = !{!1657, !1658, !1659, !1660, !1661, !1662, !1663}
!1657 = !DILocalVariable(name: "fd", arg: 1, scope: !1653, file: !588, line: 1763, type: !44)
!1658 = !DILocalVariable(name: "flags", arg: 2, scope: !1653, file: !588, line: 1764, type: !44)
!1659 = !DILocalVariable(name: "path", arg: 3, scope: !1653, file: !588, line: 1765, type: !22)
!1660 = !DILocalVariable(name: "atim", arg: 4, scope: !1653, file: !588, line: 1766, type: !360)
!1661 = !DILocalVariable(name: "mtim", arg: 5, scope: !1653, file: !588, line: 1767, type: !360)
!1662 = !DILocalVariable(name: "fst_flags", arg: 6, scope: !1653, file: !588, line: 1768, type: !12)
!1663 = !DILocalVariable(name: "ret", scope: !1664, file: !588, line: 1770, type: !631, align: 32)
!1664 = distinct !DILexicalBlock(scope: !1653, file: !588, line: 1770, column: 5)
!1665 = !DILocation(line: 1763, column: 5, scope: !1653)
!1666 = !DILocation(line: 1764, column: 5, scope: !1653)
!1667 = !DILocation(line: 1765, column: 5, scope: !1653)
!1668 = !DILocation(line: 1766, column: 5, scope: !1653)
!1669 = !DILocation(line: 1767, column: 5, scope: !1653)
!1670 = !DILocation(line: 1768, column: 5, scope: !1653)
!1671 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !1672)
!1672 = distinct !DILocation(line: 1773, column: 14, scope: !1653)
!1673 = !DILocation(line: 1773, column: 9, scope: !1653)
!1674 = !DILocation(line: 1774, column: 14, scope: !1653)
!1675 = !DILocation(line: 1777, column: 9, scope: !1653)
!1676 = !DILocation(line: 1770, column: 15, scope: !1653)
!1677 = !DILocation(line: 1770, column: 9, scope: !1664)
!1678 = !DILocation(line: 1779, column: 5, scope: !1664)
!1679 = !DILocation(line: 1780, column: 14, scope: !1664)
!1680 = !DILocation(line: 1780, column: 19, scope: !1664)
!1681 = !DILocation(line: 1781, column: 24, scope: !1664)
!1682 = !DILocation(line: 1781, column: 14, scope: !1664)
!1683 = !DILocation(line: 1781, column: 35, scope: !1664)
!1684 = !DILocation(line: 1783, column: 2, scope: !1653)
!1685 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated5Errno3raw17h718b90db54a58ce5E", scope: !602, file: !588, line: 225, type: !1686, scopeLine: 225, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1689, retainedNodes: !1690)
!1686 = !DISubroutineType(types: !1687)
!1687 = !{!12, !1688}
!1688 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Errno", baseType: !602, size: 32, align: 32, dwarfAddressSpace: 0)
!1689 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated5Errno3raw17h718b90db54a58ce5E", scope: !602, file: !588, line: 225, type: !1686, scopeLine: 225, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1690 = !{!1691}
!1691 = !DILocalVariable(name: "self", arg: 1, scope: !1685, file: !588, line: 225, type: !1688)
!1692 = !DILocation(line: 225, column: 22, scope: !1685)
!1693 = !DILocation(line: 226, column: 9, scope: !1685)
!1694 = !DILocation(line: 227, column: 6, scope: !1685)
!1695 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated5Errno4name17h3a1ad9514045c510E", scope: !602, file: !588, line: 229, type: !1696, scopeLine: 229, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1698, retainedNodes: !1699)
!1696 = !DISubroutineType(types: !1697)
!1697 = !{!22, !1688}
!1698 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated5Errno4name17h3a1ad9514045c510E", scope: !602, file: !588, line: 229, type: !1696, scopeLine: 229, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1699 = !{!1700}
!1700 = !DILocalVariable(name: "self", arg: 1, scope: !1695, file: !588, line: 229, type: !1688)
!1701 = !DILocation(line: 229, column: 17, scope: !1695)
!1702 = !DILocation(line: 230, column: 9, scope: !1695)
!1703 = !DILocation(line: 308, column: 27, scope: !1695)
!1704 = !DILocation(line: 231, column: 18, scope: !1695)
!1705 = !DILocation(line: 232, column: 18, scope: !1695)
!1706 = !DILocation(line: 233, column: 18, scope: !1695)
!1707 = !DILocation(line: 234, column: 18, scope: !1695)
!1708 = !DILocation(line: 235, column: 18, scope: !1695)
!1709 = !DILocation(line: 236, column: 18, scope: !1695)
!1710 = !DILocation(line: 237, column: 18, scope: !1695)
!1711 = !DILocation(line: 238, column: 18, scope: !1695)
!1712 = !DILocation(line: 239, column: 18, scope: !1695)
!1713 = !DILocation(line: 240, column: 18, scope: !1695)
!1714 = !DILocation(line: 241, column: 19, scope: !1695)
!1715 = !DILocation(line: 242, column: 19, scope: !1695)
!1716 = !DILocation(line: 243, column: 19, scope: !1695)
!1717 = !DILocation(line: 244, column: 19, scope: !1695)
!1718 = !DILocation(line: 245, column: 19, scope: !1695)
!1719 = !DILocation(line: 246, column: 19, scope: !1695)
!1720 = !DILocation(line: 247, column: 19, scope: !1695)
!1721 = !DILocation(line: 248, column: 19, scope: !1695)
!1722 = !DILocation(line: 249, column: 19, scope: !1695)
!1723 = !DILocation(line: 250, column: 19, scope: !1695)
!1724 = !DILocation(line: 251, column: 19, scope: !1695)
!1725 = !DILocation(line: 252, column: 19, scope: !1695)
!1726 = !DILocation(line: 253, column: 19, scope: !1695)
!1727 = !DILocation(line: 254, column: 19, scope: !1695)
!1728 = !DILocation(line: 255, column: 19, scope: !1695)
!1729 = !DILocation(line: 256, column: 19, scope: !1695)
!1730 = !DILocation(line: 257, column: 19, scope: !1695)
!1731 = !DILocation(line: 258, column: 19, scope: !1695)
!1732 = !DILocation(line: 259, column: 19, scope: !1695)
!1733 = !DILocation(line: 260, column: 19, scope: !1695)
!1734 = !DILocation(line: 261, column: 19, scope: !1695)
!1735 = !DILocation(line: 262, column: 19, scope: !1695)
!1736 = !DILocation(line: 263, column: 19, scope: !1695)
!1737 = !DILocation(line: 264, column: 19, scope: !1695)
!1738 = !DILocation(line: 265, column: 19, scope: !1695)
!1739 = !DILocation(line: 266, column: 19, scope: !1695)
!1740 = !DILocation(line: 267, column: 19, scope: !1695)
!1741 = !DILocation(line: 268, column: 19, scope: !1695)
!1742 = !DILocation(line: 269, column: 19, scope: !1695)
!1743 = !DILocation(line: 270, column: 19, scope: !1695)
!1744 = !DILocation(line: 271, column: 19, scope: !1695)
!1745 = !DILocation(line: 272, column: 19, scope: !1695)
!1746 = !DILocation(line: 273, column: 19, scope: !1695)
!1747 = !DILocation(line: 274, column: 19, scope: !1695)
!1748 = !DILocation(line: 275, column: 19, scope: !1695)
!1749 = !DILocation(line: 276, column: 19, scope: !1695)
!1750 = !DILocation(line: 277, column: 19, scope: !1695)
!1751 = !DILocation(line: 278, column: 19, scope: !1695)
!1752 = !DILocation(line: 279, column: 19, scope: !1695)
!1753 = !DILocation(line: 280, column: 19, scope: !1695)
!1754 = !DILocation(line: 281, column: 19, scope: !1695)
!1755 = !DILocation(line: 282, column: 19, scope: !1695)
!1756 = !DILocation(line: 283, column: 19, scope: !1695)
!1757 = !DILocation(line: 284, column: 19, scope: !1695)
!1758 = !DILocation(line: 285, column: 19, scope: !1695)
!1759 = !DILocation(line: 286, column: 19, scope: !1695)
!1760 = !DILocation(line: 287, column: 19, scope: !1695)
!1761 = !DILocation(line: 288, column: 19, scope: !1695)
!1762 = !DILocation(line: 289, column: 19, scope: !1695)
!1763 = !DILocation(line: 290, column: 19, scope: !1695)
!1764 = !DILocation(line: 291, column: 19, scope: !1695)
!1765 = !DILocation(line: 292, column: 19, scope: !1695)
!1766 = !DILocation(line: 293, column: 19, scope: !1695)
!1767 = !DILocation(line: 294, column: 19, scope: !1695)
!1768 = !DILocation(line: 295, column: 19, scope: !1695)
!1769 = !DILocation(line: 296, column: 19, scope: !1695)
!1770 = !DILocation(line: 297, column: 19, scope: !1695)
!1771 = !DILocation(line: 298, column: 19, scope: !1695)
!1772 = !DILocation(line: 299, column: 19, scope: !1695)
!1773 = !DILocation(line: 300, column: 19, scope: !1695)
!1774 = !DILocation(line: 301, column: 19, scope: !1695)
!1775 = !DILocation(line: 302, column: 19, scope: !1695)
!1776 = !DILocation(line: 303, column: 19, scope: !1695)
!1777 = !DILocation(line: 304, column: 19, scope: !1695)
!1778 = !DILocation(line: 305, column: 19, scope: !1695)
!1779 = !DILocation(line: 306, column: 19, scope: !1695)
!1780 = !DILocation(line: 307, column: 19, scope: !1695)
!1781 = !DILocation(line: 310, column: 6, scope: !1695)
!1782 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated5Errno7message17h331e22806c2b66acE", scope: !602, file: !588, line: 311, type: !1696, scopeLine: 311, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1783, retainedNodes: !1784)
!1783 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated5Errno7message17h331e22806c2b66acE", scope: !602, file: !588, line: 311, type: !1696, scopeLine: 311, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1784 = !{!1785}
!1785 = !DILocalVariable(name: "self", arg: 1, scope: !1782, file: !588, line: 311, type: !1688)
!1786 = !DILocation(line: 311, column: 20, scope: !1782)
!1787 = !DILocation(line: 312, column: 9, scope: !1782)
!1788 = !DILocation(line: 390, column: 27, scope: !1782)
!1789 = !DILocation(line: 313, column: 18, scope: !1782)
!1790 = !DILocation(line: 314, column: 18, scope: !1782)
!1791 = !DILocation(line: 315, column: 18, scope: !1782)
!1792 = !DILocation(line: 316, column: 18, scope: !1782)
!1793 = !DILocation(line: 317, column: 18, scope: !1782)
!1794 = !DILocation(line: 318, column: 18, scope: !1782)
!1795 = !DILocation(line: 319, column: 18, scope: !1782)
!1796 = !DILocation(line: 320, column: 18, scope: !1782)
!1797 = !DILocation(line: 321, column: 18, scope: !1782)
!1798 = !DILocation(line: 322, column: 18, scope: !1782)
!1799 = !DILocation(line: 323, column: 19, scope: !1782)
!1800 = !DILocation(line: 324, column: 19, scope: !1782)
!1801 = !DILocation(line: 325, column: 19, scope: !1782)
!1802 = !DILocation(line: 326, column: 19, scope: !1782)
!1803 = !DILocation(line: 327, column: 19, scope: !1782)
!1804 = !DILocation(line: 328, column: 19, scope: !1782)
!1805 = !DILocation(line: 329, column: 19, scope: !1782)
!1806 = !DILocation(line: 330, column: 19, scope: !1782)
!1807 = !DILocation(line: 331, column: 19, scope: !1782)
!1808 = !DILocation(line: 332, column: 19, scope: !1782)
!1809 = !DILocation(line: 333, column: 19, scope: !1782)
!1810 = !DILocation(line: 334, column: 19, scope: !1782)
!1811 = !DILocation(line: 335, column: 19, scope: !1782)
!1812 = !DILocation(line: 336, column: 19, scope: !1782)
!1813 = !DILocation(line: 337, column: 19, scope: !1782)
!1814 = !DILocation(line: 338, column: 19, scope: !1782)
!1815 = !DILocation(line: 339, column: 19, scope: !1782)
!1816 = !DILocation(line: 340, column: 19, scope: !1782)
!1817 = !DILocation(line: 341, column: 19, scope: !1782)
!1818 = !DILocation(line: 342, column: 19, scope: !1782)
!1819 = !DILocation(line: 343, column: 19, scope: !1782)
!1820 = !DILocation(line: 344, column: 19, scope: !1782)
!1821 = !DILocation(line: 345, column: 19, scope: !1782)
!1822 = !DILocation(line: 346, column: 19, scope: !1782)
!1823 = !DILocation(line: 347, column: 19, scope: !1782)
!1824 = !DILocation(line: 348, column: 19, scope: !1782)
!1825 = !DILocation(line: 349, column: 19, scope: !1782)
!1826 = !DILocation(line: 350, column: 19, scope: !1782)
!1827 = !DILocation(line: 351, column: 19, scope: !1782)
!1828 = !DILocation(line: 352, column: 19, scope: !1782)
!1829 = !DILocation(line: 353, column: 19, scope: !1782)
!1830 = !DILocation(line: 354, column: 19, scope: !1782)
!1831 = !DILocation(line: 355, column: 19, scope: !1782)
!1832 = !DILocation(line: 356, column: 19, scope: !1782)
!1833 = !DILocation(line: 357, column: 19, scope: !1782)
!1834 = !DILocation(line: 358, column: 19, scope: !1782)
!1835 = !DILocation(line: 359, column: 19, scope: !1782)
!1836 = !DILocation(line: 360, column: 19, scope: !1782)
!1837 = !DILocation(line: 361, column: 19, scope: !1782)
!1838 = !DILocation(line: 362, column: 19, scope: !1782)
!1839 = !DILocation(line: 363, column: 19, scope: !1782)
!1840 = !DILocation(line: 364, column: 19, scope: !1782)
!1841 = !DILocation(line: 365, column: 19, scope: !1782)
!1842 = !DILocation(line: 366, column: 19, scope: !1782)
!1843 = !DILocation(line: 367, column: 19, scope: !1782)
!1844 = !DILocation(line: 368, column: 19, scope: !1782)
!1845 = !DILocation(line: 369, column: 19, scope: !1782)
!1846 = !DILocation(line: 370, column: 19, scope: !1782)
!1847 = !DILocation(line: 371, column: 19, scope: !1782)
!1848 = !DILocation(line: 372, column: 19, scope: !1782)
!1849 = !DILocation(line: 373, column: 19, scope: !1782)
!1850 = !DILocation(line: 374, column: 19, scope: !1782)
!1851 = !DILocation(line: 375, column: 19, scope: !1782)
!1852 = !DILocation(line: 376, column: 19, scope: !1782)
!1853 = !DILocation(line: 377, column: 19, scope: !1782)
!1854 = !DILocation(line: 378, column: 19, scope: !1782)
!1855 = !DILocation(line: 379, column: 19, scope: !1782)
!1856 = !DILocation(line: 380, column: 19, scope: !1782)
!1857 = !DILocation(line: 381, column: 19, scope: !1782)
!1858 = !DILocation(line: 382, column: 19, scope: !1782)
!1859 = !DILocation(line: 383, column: 19, scope: !1782)
!1860 = !DILocation(line: 384, column: 19, scope: !1782)
!1861 = !DILocation(line: 385, column: 19, scope: !1782)
!1862 = !DILocation(line: 386, column: 19, scope: !1782)
!1863 = !DILocation(line: 387, column: 19, scope: !1782)
!1864 = !DILocation(line: 388, column: 19, scope: !1782)
!1865 = !DILocation(line: 389, column: 19, scope: !1782)
!1866 = !DILocation(line: 392, column: 6, scope: !1782)
!1867 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Advice3raw17h15896fee8c180decE", scope: !1868, file: !588, line: 631, type: !1871, scopeLine: 631, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1874, retainedNodes: !1875)
!1868 = !DICompositeType(tag: DW_TAG_structure_type, name: "Advice", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !1869, templateParams: !13, identifier: "83206ec9fc8f8bff8c3450d0e2b8af1")
!1869 = !{!1870}
!1870 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1868, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!1871 = !DISubroutineType(types: !1872)
!1872 = !{!26, !1873}
!1873 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Advice", baseType: !1868, size: 32, align: 32, dwarfAddressSpace: 0)
!1874 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Advice3raw17h15896fee8c180decE", scope: !1868, file: !588, line: 631, type: !1871, scopeLine: 631, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1875 = !{!1876}
!1876 = !DILocalVariable(name: "self", arg: 1, scope: !1867, file: !588, line: 631, type: !1873)
!1877 = !DILocation(line: 631, column: 22, scope: !1867)
!1878 = !DILocation(line: 632, column: 9, scope: !1867)
!1879 = !DILocation(line: 633, column: 6, scope: !1867)
!1880 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Advice4name17h144c54f5ff34b5e2E", scope: !1868, file: !588, line: 635, type: !1881, scopeLine: 635, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1883, retainedNodes: !1884)
!1881 = !DISubroutineType(types: !1882)
!1882 = !{!22, !1873}
!1883 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Advice4name17h144c54f5ff34b5e2E", scope: !1868, file: !588, line: 635, type: !1881, scopeLine: 635, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1884 = !{!1885}
!1885 = !DILocalVariable(name: "self", arg: 1, scope: !1880, file: !588, line: 635, type: !1873)
!1886 = !DILocation(line: 635, column: 17, scope: !1880)
!1887 = !DILocation(line: 636, column: 9, scope: !1880)
!1888 = !DILocation(line: 643, column: 27, scope: !1880)
!1889 = !DILocation(line: 637, column: 18, scope: !1880)
!1890 = !DILocation(line: 638, column: 18, scope: !1880)
!1891 = !DILocation(line: 639, column: 18, scope: !1880)
!1892 = !DILocation(line: 640, column: 18, scope: !1880)
!1893 = !DILocation(line: 641, column: 18, scope: !1880)
!1894 = !DILocation(line: 642, column: 18, scope: !1880)
!1895 = !DILocation(line: 645, column: 6, scope: !1880)
!1896 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Advice7message17h7a17fd969790b8c7E", scope: !1868, file: !588, line: 646, type: !1881, scopeLine: 646, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1897, retainedNodes: !1898)
!1897 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Advice7message17h7a17fd969790b8c7E", scope: !1868, file: !588, line: 646, type: !1881, scopeLine: 646, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1898 = !{!1899}
!1899 = !DILocalVariable(name: "self", arg: 1, scope: !1896, file: !588, line: 646, type: !1873)
!1900 = !DILocation(line: 646, column: 20, scope: !1896)
!1901 = !DILocation(line: 647, column: 9, scope: !1896)
!1902 = !DILocation(line: 647, column: 597, scope: !1896)
!1903 = !DILocation(line: 647, column: 28, scope: !1896)
!1904 = !DILocation(line: 647, column: 125, scope: !1896)
!1905 = !DILocation(line: 647, column: 236, scope: !1896)
!1906 = !DILocation(line: 647, column: 315, scope: !1896)
!1907 = !DILocation(line: 647, column: 395, scope: !1896)
!1908 = !DILocation(line: 647, column: 489, scope: !1896)
!1909 = !DILocation(line: 648, column: 6, scope: !1896)
!1910 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Signal3raw17hf6c9fc427e6cf57fE", scope: !681, file: !588, line: 960, type: !1911, scopeLine: 960, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1914, retainedNodes: !1915)
!1911 = !DISubroutineType(types: !1912)
!1912 = !{!26, !1913}
!1913 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Signal", baseType: !681, size: 32, align: 32, dwarfAddressSpace: 0)
!1914 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Signal3raw17hf6c9fc427e6cf57fE", scope: !681, file: !588, line: 960, type: !1911, scopeLine: 960, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1915 = !{!1916}
!1916 = !DILocalVariable(name: "self", arg: 1, scope: !1910, file: !588, line: 960, type: !1913)
!1917 = !DILocation(line: 960, column: 22, scope: !1910)
!1918 = !DILocation(line: 961, column: 9, scope: !1910)
!1919 = !DILocation(line: 962, column: 6, scope: !1910)
!1920 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Signal4name17h44b7507ff21226fcE", scope: !681, file: !588, line: 964, type: !1921, scopeLine: 964, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1923, retainedNodes: !1924)
!1921 = !DISubroutineType(types: !1922)
!1922 = !{!22, !1913}
!1923 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Signal4name17h44b7507ff21226fcE", scope: !681, file: !588, line: 964, type: !1921, scopeLine: 964, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1924 = !{!1925}
!1925 = !DILocalVariable(name: "self", arg: 1, scope: !1920, file: !588, line: 964, type: !1913)
!1926 = !DILocation(line: 964, column: 17, scope: !1920)
!1927 = !DILocation(line: 965, column: 9, scope: !1920)
!1928 = !DILocation(line: 997, column: 27, scope: !1920)
!1929 = !DILocation(line: 966, column: 18, scope: !1920)
!1930 = !DILocation(line: 967, column: 18, scope: !1920)
!1931 = !DILocation(line: 968, column: 18, scope: !1920)
!1932 = !DILocation(line: 969, column: 18, scope: !1920)
!1933 = !DILocation(line: 970, column: 18, scope: !1920)
!1934 = !DILocation(line: 971, column: 18, scope: !1920)
!1935 = !DILocation(line: 972, column: 18, scope: !1920)
!1936 = !DILocation(line: 973, column: 18, scope: !1920)
!1937 = !DILocation(line: 974, column: 18, scope: !1920)
!1938 = !DILocation(line: 975, column: 18, scope: !1920)
!1939 = !DILocation(line: 976, column: 19, scope: !1920)
!1940 = !DILocation(line: 977, column: 19, scope: !1920)
!1941 = !DILocation(line: 978, column: 19, scope: !1920)
!1942 = !DILocation(line: 979, column: 19, scope: !1920)
!1943 = !DILocation(line: 980, column: 19, scope: !1920)
!1944 = !DILocation(line: 981, column: 19, scope: !1920)
!1945 = !DILocation(line: 982, column: 19, scope: !1920)
!1946 = !DILocation(line: 983, column: 19, scope: !1920)
!1947 = !DILocation(line: 984, column: 19, scope: !1920)
!1948 = !DILocation(line: 985, column: 19, scope: !1920)
!1949 = !DILocation(line: 986, column: 19, scope: !1920)
!1950 = !DILocation(line: 987, column: 19, scope: !1920)
!1951 = !DILocation(line: 988, column: 19, scope: !1920)
!1952 = !DILocation(line: 989, column: 19, scope: !1920)
!1953 = !DILocation(line: 990, column: 19, scope: !1920)
!1954 = !DILocation(line: 991, column: 19, scope: !1920)
!1955 = !DILocation(line: 992, column: 19, scope: !1920)
!1956 = !DILocation(line: 993, column: 19, scope: !1920)
!1957 = !DILocation(line: 994, column: 19, scope: !1920)
!1958 = !DILocation(line: 995, column: 19, scope: !1920)
!1959 = !DILocation(line: 996, column: 19, scope: !1920)
!1960 = !DILocation(line: 999, column: 6, scope: !1920)
!1961 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Signal7message17h5c961ce5e1cd5f15E", scope: !681, file: !588, line: 1000, type: !1921, scopeLine: 1000, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !1962, retainedNodes: !1963)
!1962 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Signal7message17h5c961ce5e1cd5f15E", scope: !681, file: !588, line: 1000, type: !1921, scopeLine: 1000, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!1963 = !{!1964}
!1964 = !DILocalVariable(name: "self", arg: 1, scope: !1961, file: !588, line: 1000, type: !1913)
!1965 = !DILocation(line: 1000, column: 20, scope: !1961)
!1966 = !DILocation(line: 1001, column: 9, scope: !1961)
!1967 = !DILocation(line: 1126, column: 27, scope: !1961)
!1968 = !DILocation(line: 1003, column: 17, scope: !1961)
!1969 = !DILocation(line: 1007, column: 17, scope: !1961)
!1970 = !DILocation(line: 1011, column: 17, scope: !1961)
!1971 = !DILocation(line: 1015, column: 17, scope: !1961)
!1972 = !DILocation(line: 1019, column: 17, scope: !1961)
!1973 = !DILocation(line: 1023, column: 17, scope: !1961)
!1974 = !DILocation(line: 1027, column: 17, scope: !1961)
!1975 = !DILocation(line: 1031, column: 17, scope: !1961)
!1976 = !DILocation(line: 1035, column: 17, scope: !1961)
!1977 = !DILocation(line: 1039, column: 17, scope: !1961)
!1978 = !DILocation(line: 1043, column: 17, scope: !1961)
!1979 = !DILocation(line: 1047, column: 17, scope: !1961)
!1980 = !DILocation(line: 1051, column: 17, scope: !1961)
!1981 = !DILocation(line: 1055, column: 17, scope: !1961)
!1982 = !DILocation(line: 1059, column: 17, scope: !1961)
!1983 = !DILocation(line: 1063, column: 17, scope: !1961)
!1984 = !DILocation(line: 1067, column: 17, scope: !1961)
!1985 = !DILocation(line: 1071, column: 17, scope: !1961)
!1986 = !DILocation(line: 1075, column: 17, scope: !1961)
!1987 = !DILocation(line: 1079, column: 17, scope: !1961)
!1988 = !DILocation(line: 1083, column: 17, scope: !1961)
!1989 = !DILocation(line: 1087, column: 17, scope: !1961)
!1990 = !DILocation(line: 1091, column: 17, scope: !1961)
!1991 = !DILocation(line: 1095, column: 17, scope: !1961)
!1992 = !DILocation(line: 1099, column: 17, scope: !1961)
!1993 = !DILocation(line: 1103, column: 17, scope: !1961)
!1994 = !DILocation(line: 1107, column: 17, scope: !1961)
!1995 = !DILocation(line: 1111, column: 17, scope: !1961)
!1996 = !DILocation(line: 1115, column: 17, scope: !1961)
!1997 = !DILocation(line: 1119, column: 17, scope: !1961)
!1998 = !DILocation(line: 1123, column: 17, scope: !1961)
!1999 = !DILocation(line: 1128, column: 6, scope: !1961)
!2000 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Whence3raw17ha742762abb60085dE", scope: !2001, file: !588, line: 518, type: !2004, scopeLine: 518, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2007, retainedNodes: !2008)
!2001 = !DICompositeType(tag: DW_TAG_structure_type, name: "Whence", scope: !351, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !2002, templateParams: !13, identifier: "34437d856f0749b3705c03d386a383e1")
!2002 = !{!2003}
!2003 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2001, file: !2, baseType: !26, size: 8, align: 8, flags: DIFlagPrivate)
!2004 = !DISubroutineType(types: !2005)
!2005 = !{!26, !2006}
!2006 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Whence", baseType: !2001, size: 32, align: 32, dwarfAddressSpace: 0)
!2007 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated6Whence3raw17ha742762abb60085dE", scope: !2001, file: !588, line: 518, type: !2004, scopeLine: 518, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2008 = !{!2009}
!2009 = !DILocalVariable(name: "self", arg: 1, scope: !2000, file: !588, line: 518, type: !2006)
!2010 = !DILocation(line: 518, column: 22, scope: !2000)
!2011 = !DILocation(line: 519, column: 9, scope: !2000)
!2012 = !DILocation(line: 520, column: 6, scope: !2000)
!2013 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Whence4name17he958b0be86dcb2e2E", scope: !2001, file: !588, line: 522, type: !2014, scopeLine: 522, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2016, retainedNodes: !2017)
!2014 = !DISubroutineType(types: !2015)
!2015 = !{!22, !2006}
!2016 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated6Whence4name17he958b0be86dcb2e2E", scope: !2001, file: !588, line: 522, type: !2014, scopeLine: 522, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2017 = !{!2018}
!2018 = !DILocalVariable(name: "self", arg: 1, scope: !2013, file: !588, line: 522, type: !2006)
!2019 = !DILocation(line: 522, column: 17, scope: !2013)
!2020 = !DILocation(line: 523, column: 9, scope: !2013)
!2021 = !DILocation(line: 527, column: 27, scope: !2013)
!2022 = !DILocation(line: 524, column: 18, scope: !2013)
!2023 = !DILocation(line: 525, column: 18, scope: !2013)
!2024 = !DILocation(line: 526, column: 18, scope: !2013)
!2025 = !DILocation(line: 529, column: 6, scope: !2013)
!2026 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Whence7message17ha9e612d937001f4fE", scope: !2001, file: !588, line: 530, type: !2014, scopeLine: 530, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2027, retainedNodes: !2028)
!2027 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated6Whence7message17ha9e612d937001f4fE", scope: !2001, file: !588, line: 530, type: !2014, scopeLine: 530, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2028 = !{!2029}
!2029 = !DILocalVariable(name: "self", arg: 1, scope: !2026, file: !588, line: 530, type: !2006)
!2030 = !DILocation(line: 530, column: 20, scope: !2026)
!2031 = !DILocation(line: 531, column: 9, scope: !2026)
!2032 = !DILocation(line: 535, column: 27, scope: !2026)
!2033 = !DILocation(line: 532, column: 18, scope: !2026)
!2034 = !DILocation(line: 533, column: 18, scope: !2026)
!2035 = !DILocation(line: 534, column: 18, scope: !2026)
!2036 = !DILocation(line: 537, column: 6, scope: !2026)
!2037 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated7Clockid3raw17h3b57b00042d58b44E", scope: !884, file: !588, line: 26, type: !2038, scopeLine: 26, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2041, retainedNodes: !2042)
!2038 = !DISubroutineType(types: !2039)
!2039 = !{!44, !2040}
!2040 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Clockid", baseType: !884, size: 32, align: 32, dwarfAddressSpace: 0)
!2041 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated7Clockid3raw17h3b57b00042d58b44E", scope: !884, file: !588, line: 26, type: !2038, scopeLine: 26, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2042 = !{!2043}
!2043 = !DILocalVariable(name: "self", arg: 1, scope: !2037, file: !588, line: 26, type: !2040)
!2044 = !DILocation(line: 26, column: 22, scope: !2037)
!2045 = !DILocation(line: 27, column: 9, scope: !2037)
!2046 = !DILocation(line: 28, column: 6, scope: !2037)
!2047 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated7Clockid4name17h908f75bcb5817523E", scope: !884, file: !588, line: 30, type: !2048, scopeLine: 30, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2050, retainedNodes: !2051)
!2048 = !DISubroutineType(types: !2049)
!2049 = !{!22, !2040}
!2050 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated7Clockid4name17h908f75bcb5817523E", scope: !884, file: !588, line: 30, type: !2048, scopeLine: 30, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2051 = !{!2052}
!2052 = !DILocalVariable(name: "self", arg: 1, scope: !2047, file: !588, line: 30, type: !2040)
!2053 = !DILocation(line: 30, column: 17, scope: !2047)
!2054 = !DILocation(line: 31, column: 9, scope: !2047)
!2055 = !DILocation(line: 36, column: 27, scope: !2047)
!2056 = !DILocation(line: 32, column: 18, scope: !2047)
!2057 = !DILocation(line: 33, column: 18, scope: !2047)
!2058 = !DILocation(line: 34, column: 18, scope: !2047)
!2059 = !DILocation(line: 35, column: 18, scope: !2047)
!2060 = !DILocation(line: 38, column: 6, scope: !2047)
!2061 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated7Clockid7message17hf15037538852fe88E", scope: !884, file: !588, line: 39, type: !2048, scopeLine: 39, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2062, retainedNodes: !2063)
!2062 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated7Clockid7message17hf15037538852fe88E", scope: !884, file: !588, line: 39, type: !2048, scopeLine: 39, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2063 = !{!2064}
!2064 = !DILocalVariable(name: "self", arg: 1, scope: !2061, file: !588, line: 39, type: !2040)
!2065 = !DILocation(line: 39, column: 20, scope: !2061)
!2066 = !DILocation(line: 40, column: 9, scope: !2061)
!2067 = !DILocation(line: 53, column: 27, scope: !2061)
!2068 = !DILocation(line: 42, column: 17, scope: !2061)
!2069 = !DILocation(line: 46, column: 17, scope: !2061)
!2070 = !DILocation(line: 51, column: 18, scope: !2061)
!2071 = !DILocation(line: 52, column: 18, scope: !2061)
!2072 = !DILocation(line: 55, column: 6, scope: !2061)
!2073 = distinct !DISubprogram(name: "fd_read", linkageName: "_ZN4wasi13lib_generated7fd_read17hb0dacab1222e8d36E", scope: !351, file: !588, line: 1566, type: !2074, scopeLine: 1566, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2085)
!2074 = !DISubroutineType(types: !2075)
!2075 = !{!591, !44, !2076}
!2076 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[wasi::lib_generated::Iovec]", file: !2, size: 64, align: 32, elements: !2077, templateParams: !13, identifier: "c032ffd0def6b4cfb4459f80a9d37ac0")
!2077 = !{!2078, !2084}
!2078 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2076, file: !2, baseType: !2079, size: 32, align: 32)
!2079 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !2080, size: 32, align: 32, dwarfAddressSpace: 0)
!2080 = !DICompositeType(tag: DW_TAG_structure_type, name: "Iovec", scope: !351, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !2081, templateParams: !13, identifier: "4d7e8f7e29951817add797fba3410c8d")
!2081 = !{!2082, !2083}
!2082 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !2080, file: !2, baseType: !610, size: 32, align: 32, flags: DIFlagPublic)
!2083 = !DIDerivedType(tag: DW_TAG_member, name: "buf_len", scope: !2080, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!2084 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2076, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!2085 = !{!2086, !2087, !2088, !2090}
!2086 = !DILocalVariable(name: "fd", arg: 1, scope: !2073, file: !588, line: 1566, type: !44)
!2087 = !DILocalVariable(name: "iovs", arg: 2, scope: !2073, file: !588, line: 1566, type: !2076)
!2088 = !DILocalVariable(name: "rp0", scope: !2089, file: !588, line: 1567, type: !618, align: 32)
!2089 = distinct !DILexicalBlock(scope: !2073, file: !588, line: 1567, column: 5)
!2090 = !DILocalVariable(name: "ret", scope: !2091, file: !588, line: 1568, type: !631, align: 32)
!2091 = distinct !DILexicalBlock(scope: !2089, file: !588, line: 1568, column: 5)
!2092 = !DILocation(line: 1566, column: 23, scope: !2073)
!2093 = !DILocation(line: 1566, column: 31, scope: !2073)
!2094 = !DILocation(line: 1567, column: 9, scope: !2089)
!2095 = !DILocation(line: 1567, column: 19, scope: !2073)
!2096 = !DILocalVariable(name: "self", arg: 1, scope: !2097, file: !2098, line: 724, type: !2076)
!2097 = distinct !DISubprogram(name: "as_ptr<wasi::lib_generated::Iovec>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17hdfb8163dae8cec7eE", scope: !2099, file: !2098, line: 724, type: !2101, scopeLine: 724, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !2105, retainedNodes: !2104)
!2098 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "63aedd801a9e6eae1eca1edc5c2217aa")
!2099 = !DINamespace(name: "{impl#0}", scope: !2100)
!2100 = !DINamespace(name: "slice", scope: !56)
!2101 = !DISubroutineType(types: !2102)
!2102 = !{!2103, !2076}
!2103 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Iovec", baseType: !2080, size: 32, align: 32, dwarfAddressSpace: 0)
!2104 = !{!2096}
!2105 = !{!2106}
!2106 = !DITemplateTypeParameter(name: "T", type: !2080)
!2107 = !DILocation(line: 724, column: 25, scope: !2097, inlinedAt: !2108)
!2108 = distinct !DILocation(line: 1570, column: 14, scope: !2089)
!2109 = !DILocation(line: 1570, column: 9, scope: !2089)
!2110 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2111)
!2111 = distinct !DILocation(line: 1572, column: 13, scope: !2089)
!2112 = !DILocation(line: 1572, column: 9, scope: !2089)
!2113 = !DILocation(line: 1568, column: 15, scope: !2089)
!2114 = !DILocation(line: 1568, column: 9, scope: !2091)
!2115 = !DILocation(line: 1574, column: 5, scope: !2091)
!2116 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2117)
!2117 = distinct !DILocation(line: 1575, column: 37, scope: !2091)
!2118 = !DILocation(line: 1575, column: 33, scope: !2091)
!2119 = !DILocation(line: 1575, column: 17, scope: !2091)
!2120 = !DILocation(line: 1575, column: 14, scope: !2091)
!2121 = !DILocation(line: 1575, column: 72, scope: !2091)
!2122 = !DILocation(line: 1576, column: 24, scope: !2091)
!2123 = !DILocation(line: 1576, column: 14, scope: !2091)
!2124 = !DILocation(line: 1576, column: 35, scope: !2091)
!2125 = !DILocation(line: 1578, column: 2, scope: !2073)
!2126 = distinct !DISubprogram(name: "fd_seek", linkageName: "_ZN4wasi13lib_generated7fd_seek17h9e7fb5fc4b342c97E", scope: !351, file: !588, line: 1649, type: !2127, scopeLine: 1649, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2130)
!2127 = !DISubroutineType(types: !2128)
!2128 = !{!1047, !44, !2129, !2001}
!2129 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!2130 = !{!2131, !2132, !2133, !2134, !2136}
!2131 = !DILocalVariable(name: "fd", arg: 1, scope: !2126, file: !588, line: 1649, type: !44)
!2132 = !DILocalVariable(name: "offset", arg: 2, scope: !2126, file: !588, line: 1649, type: !2129)
!2133 = !DILocalVariable(name: "whence", arg: 3, scope: !2126, file: !588, line: 1649, type: !2001)
!2134 = !DILocalVariable(name: "rp0", scope: !2135, file: !588, line: 1650, type: !1065, align: 64)
!2135 = distinct !DILexicalBlock(scope: !2126, file: !588, line: 1650, column: 5)
!2136 = !DILocalVariable(name: "ret", scope: !2137, file: !588, line: 1651, type: !631, align: 32)
!2137 = distinct !DILexicalBlock(scope: !2135, file: !588, line: 1651, column: 5)
!2138 = !DILocation(line: 1649, column: 23, scope: !2126)
!2139 = !DILocation(line: 1649, column: 31, scope: !2126)
!2140 = !DILocation(line: 1649, column: 50, scope: !2126)
!2141 = !DILocation(line: 1650, column: 9, scope: !2135)
!2142 = !DILocation(line: 1650, column: 19, scope: !2126)
!2143 = !DILocation(line: 1654, column: 9, scope: !2135)
!2144 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !2145)
!2145 = distinct !DILocation(line: 1655, column: 13, scope: !2135)
!2146 = !DILocation(line: 1655, column: 9, scope: !2135)
!2147 = !DILocation(line: 1651, column: 15, scope: !2135)
!2148 = !DILocation(line: 1651, column: 9, scope: !2137)
!2149 = !DILocation(line: 1657, column: 5, scope: !2137)
!2150 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !2151)
!2151 = distinct !DILocation(line: 1658, column: 37, scope: !2137)
!2152 = !DILocation(line: 1658, column: 33, scope: !2137)
!2153 = !DILocation(line: 1658, column: 17, scope: !2137)
!2154 = !DILocation(line: 1658, column: 14, scope: !2137)
!2155 = !DILocation(line: 1658, column: 76, scope: !2137)
!2156 = !DILocation(line: 1659, column: 24, scope: !2137)
!2157 = !DILocation(line: 1659, column: 14, scope: !2137)
!2158 = !DILocation(line: 1659, column: 35, scope: !2137)
!2159 = !DILocation(line: 1661, column: 2, scope: !2126)
!2160 = distinct !DISubprogram(name: "fd_sync", linkageName: "_ZN4wasi13lib_generated7fd_sync17h0245601cea21c804E", scope: !351, file: !588, line: 1665, type: !794, scopeLine: 1665, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2161)
!2161 = !{!2162, !2163}
!2162 = !DILocalVariable(name: "fd", arg: 1, scope: !2160, file: !588, line: 1665, type: !44)
!2163 = !DILocalVariable(name: "ret", scope: !2164, file: !588, line: 1666, type: !631, align: 32)
!2164 = distinct !DILexicalBlock(scope: !2160, file: !588, line: 1666, column: 5)
!2165 = !DILocation(line: 1665, column: 23, scope: !2160)
!2166 = !DILocation(line: 1666, column: 15, scope: !2160)
!2167 = !DILocation(line: 1666, column: 9, scope: !2164)
!2168 = !DILocation(line: 1667, column: 5, scope: !2164)
!2169 = !DILocation(line: 1668, column: 14, scope: !2164)
!2170 = !DILocation(line: 1668, column: 19, scope: !2164)
!2171 = !DILocation(line: 1669, column: 24, scope: !2164)
!2172 = !DILocation(line: 1669, column: 14, scope: !2164)
!2173 = !DILocation(line: 1669, column: 35, scope: !2164)
!2174 = !DILocation(line: 1671, column: 2, scope: !2160)
!2175 = distinct !DISubprogram(name: "fd_tell", linkageName: "_ZN4wasi13lib_generated7fd_tell17h7dc9dda5a5637ea0E", scope: !351, file: !588, line: 1679, type: !2176, scopeLine: 1679, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2178)
!2176 = !DISubroutineType(types: !2177)
!2177 = !{!1047, !44}
!2178 = !{!2179, !2180, !2182}
!2179 = !DILocalVariable(name: "fd", arg: 1, scope: !2175, file: !588, line: 1679, type: !44)
!2180 = !DILocalVariable(name: "rp0", scope: !2181, file: !588, line: 1680, type: !1065, align: 64)
!2181 = distinct !DILexicalBlock(scope: !2175, file: !588, line: 1680, column: 5)
!2182 = !DILocalVariable(name: "ret", scope: !2183, file: !588, line: 1681, type: !631, align: 32)
!2183 = distinct !DILexicalBlock(scope: !2181, file: !588, line: 1681, column: 5)
!2184 = !DILocation(line: 1679, column: 23, scope: !2175)
!2185 = !DILocation(line: 1680, column: 9, scope: !2181)
!2186 = !DILocation(line: 1680, column: 19, scope: !2175)
!2187 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !2188)
!2188 = distinct !DILocation(line: 1681, column: 62, scope: !2181)
!2189 = !DILocation(line: 1681, column: 58, scope: !2181)
!2190 = !DILocation(line: 1681, column: 15, scope: !2181)
!2191 = !DILocation(line: 1681, column: 9, scope: !2183)
!2192 = !DILocation(line: 1682, column: 5, scope: !2183)
!2193 = !DILocation(line: 560, column: 29, scope: !1078, inlinedAt: !2194)
!2194 = distinct !DILocation(line: 1683, column: 37, scope: !2183)
!2195 = !DILocation(line: 1683, column: 33, scope: !2183)
!2196 = !DILocation(line: 1683, column: 17, scope: !2183)
!2197 = !DILocation(line: 1683, column: 14, scope: !2183)
!2198 = !DILocation(line: 1683, column: 76, scope: !2183)
!2199 = !DILocation(line: 1684, column: 24, scope: !2183)
!2200 = !DILocation(line: 1684, column: 14, scope: !2183)
!2201 = !DILocation(line: 1684, column: 35, scope: !2183)
!2202 = !DILocation(line: 1686, column: 2, scope: !2175)
!2203 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated8Filetype3raw17h3070f49fd500d0dfE", scope: !355, file: !588, line: 572, type: !2204, scopeLine: 572, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2207, retainedNodes: !2208)
!2204 = !DISubroutineType(types: !2205)
!2205 = !{!26, !2206}
!2206 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Filetype", baseType: !355, size: 32, align: 32, dwarfAddressSpace: 0)
!2207 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated8Filetype3raw17h3070f49fd500d0dfE", scope: !355, file: !588, line: 572, type: !2204, scopeLine: 572, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2208 = !{!2209}
!2209 = !DILocalVariable(name: "self", arg: 1, scope: !2203, file: !588, line: 572, type: !2206)
!2210 = !DILocation(line: 572, column: 22, scope: !2203)
!2211 = !DILocation(line: 573, column: 9, scope: !2203)
!2212 = !DILocation(line: 574, column: 6, scope: !2203)
!2213 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated8Filetype4name17h42a52d431e4e0e3fE", scope: !355, file: !588, line: 576, type: !2214, scopeLine: 576, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2216, retainedNodes: !2217)
!2214 = !DISubroutineType(types: !2215)
!2215 = !{!22, !2206}
!2216 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated8Filetype4name17h42a52d431e4e0e3fE", scope: !355, file: !588, line: 576, type: !2214, scopeLine: 576, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2217 = !{!2218}
!2218 = !DILocalVariable(name: "self", arg: 1, scope: !2213, file: !588, line: 576, type: !2206)
!2219 = !DILocation(line: 576, column: 17, scope: !2213)
!2220 = !DILocation(line: 577, column: 9, scope: !2213)
!2221 = !DILocation(line: 586, column: 27, scope: !2213)
!2222 = !DILocation(line: 578, column: 18, scope: !2213)
!2223 = !DILocation(line: 579, column: 18, scope: !2213)
!2224 = !DILocation(line: 580, column: 18, scope: !2213)
!2225 = !DILocation(line: 581, column: 18, scope: !2213)
!2226 = !DILocation(line: 582, column: 18, scope: !2213)
!2227 = !DILocation(line: 583, column: 18, scope: !2213)
!2228 = !DILocation(line: 584, column: 18, scope: !2213)
!2229 = !DILocation(line: 585, column: 18, scope: !2213)
!2230 = !DILocation(line: 588, column: 6, scope: !2213)
!2231 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated8Filetype7message17hc28156d203146fb1E", scope: !355, file: !588, line: 589, type: !2214, scopeLine: 589, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2232, retainedNodes: !2233)
!2232 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated8Filetype7message17hc28156d203146fb1E", scope: !355, file: !588, line: 589, type: !2214, scopeLine: 589, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2233 = !{!2234}
!2234 = !DILocalVariable(name: "self", arg: 1, scope: !2231, file: !588, line: 589, type: !2206)
!2235 = !DILocation(line: 589, column: 20, scope: !2231)
!2236 = !DILocation(line: 590, column: 9, scope: !2231)
!2237 = !DILocation(line: 590, column: 599, scope: !2231)
!2238 = !DILocation(line: 590, column: 28, scope: !2231)
!2239 = !DILocation(line: 590, column: 141, scope: !2231)
!2240 = !DILocation(line: 590, column: 208, scope: !2231)
!2241 = !DILocation(line: 590, column: 279, scope: !2231)
!2242 = !DILocation(line: 590, column: 343, scope: !2231)
!2243 = !DILocation(line: 590, column: 410, scope: !2231)
!2244 = !DILocation(line: 590, column: 474, scope: !2231)
!2245 = !DILocation(line: 590, column: 541, scope: !2231)
!2246 = !DILocation(line: 591, column: 6, scope: !2231)
!2247 = distinct !DISubprogram(name: "args_get", linkageName: "_ZN4wasi13lib_generated8args_get17hfd25f001ff1fe64aE", scope: !351, file: !588, line: 1211, type: !751, scopeLine: 1211, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2248)
!2248 = !{!2249, !2250, !2251}
!2249 = !DILocalVariable(name: "argv", arg: 1, scope: !2247, file: !588, line: 1211, type: !753)
!2250 = !DILocalVariable(name: "argv_buf", arg: 2, scope: !2247, file: !588, line: 1211, type: !610)
!2251 = !DILocalVariable(name: "ret", scope: !2252, file: !588, line: 1212, type: !631, align: 32)
!2252 = distinct !DILexicalBlock(scope: !2247, file: !588, line: 1212, column: 5)
!2253 = !DILocation(line: 1211, column: 24, scope: !2247)
!2254 = !DILocation(line: 1211, column: 44, scope: !2247)
!2255 = !DILocation(line: 1212, column: 48, scope: !2247)
!2256 = !DILocation(line: 1212, column: 61, scope: !2247)
!2257 = !DILocation(line: 1212, column: 15, scope: !2247)
!2258 = !DILocation(line: 1212, column: 9, scope: !2252)
!2259 = !DILocation(line: 1213, column: 5, scope: !2252)
!2260 = !DILocation(line: 1214, column: 14, scope: !2252)
!2261 = !DILocation(line: 1214, column: 19, scope: !2252)
!2262 = !DILocation(line: 1215, column: 24, scope: !2252)
!2263 = !DILocation(line: 1215, column: 14, scope: !2252)
!2264 = !DILocation(line: 1215, column: 35, scope: !2252)
!2265 = !DILocation(line: 1217, column: 2, scope: !2247)
!2266 = distinct !DISubprogram(name: "fd_close", linkageName: "_ZN4wasi13lib_generated8fd_close17hc2388cad40911762E", scope: !351, file: !588, line: 1354, type: !794, scopeLine: 1354, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2267)
!2267 = !{!2268, !2269}
!2268 = !DILocalVariable(name: "fd", arg: 1, scope: !2266, file: !588, line: 1354, type: !44)
!2269 = !DILocalVariable(name: "ret", scope: !2270, file: !588, line: 1355, type: !631, align: 32)
!2270 = distinct !DILexicalBlock(scope: !2266, file: !588, line: 1355, column: 5)
!2271 = !DILocation(line: 1354, column: 24, scope: !2266)
!2272 = !DILocation(line: 1355, column: 15, scope: !2266)
!2273 = !DILocation(line: 1355, column: 9, scope: !2270)
!2274 = !DILocation(line: 1356, column: 5, scope: !2270)
!2275 = !DILocation(line: 1357, column: 14, scope: !2270)
!2276 = !DILocation(line: 1357, column: 19, scope: !2270)
!2277 = !DILocation(line: 1358, column: 24, scope: !2270)
!2278 = !DILocation(line: 1358, column: 14, scope: !2270)
!2279 = !DILocation(line: 1358, column: 35, scope: !2270)
!2280 = !DILocation(line: 1360, column: 2, scope: !2266)
!2281 = distinct !DISubprogram(name: "fd_pread", linkageName: "_ZN4wasi13lib_generated8fd_pread17hc381ec5dadb084ceE", scope: !351, file: !588, line: 1488, type: !2282, scopeLine: 1488, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2284)
!2282 = !DISubroutineType(types: !2283)
!2283 = !{!591, !44, !2076, !360}
!2284 = !{!2285, !2286, !2287, !2288, !2290}
!2285 = !DILocalVariable(name: "fd", arg: 1, scope: !2281, file: !588, line: 1488, type: !44)
!2286 = !DILocalVariable(name: "iovs", arg: 2, scope: !2281, file: !588, line: 1488, type: !2076)
!2287 = !DILocalVariable(name: "offset", arg: 3, scope: !2281, file: !588, line: 1488, type: !360)
!2288 = !DILocalVariable(name: "rp0", scope: !2289, file: !588, line: 1489, type: !618, align: 32)
!2289 = distinct !DILexicalBlock(scope: !2281, file: !588, line: 1489, column: 5)
!2290 = !DILocalVariable(name: "ret", scope: !2291, file: !588, line: 1490, type: !631, align: 32)
!2291 = distinct !DILexicalBlock(scope: !2289, file: !588, line: 1490, column: 5)
!2292 = !DILocation(line: 1488, column: 24, scope: !2281)
!2293 = !DILocation(line: 1488, column: 32, scope: !2281)
!2294 = !DILocation(line: 1488, column: 54, scope: !2281)
!2295 = !DILocation(line: 1489, column: 9, scope: !2289)
!2296 = !DILocation(line: 1489, column: 19, scope: !2281)
!2297 = !DILocation(line: 724, column: 25, scope: !2097, inlinedAt: !2298)
!2298 = distinct !DILocation(line: 1492, column: 14, scope: !2289)
!2299 = !DILocation(line: 1492, column: 9, scope: !2289)
!2300 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2301)
!2301 = distinct !DILocation(line: 1495, column: 13, scope: !2289)
!2302 = !DILocation(line: 1495, column: 9, scope: !2289)
!2303 = !DILocation(line: 1490, column: 15, scope: !2289)
!2304 = !DILocation(line: 1490, column: 9, scope: !2291)
!2305 = !DILocation(line: 1497, column: 5, scope: !2291)
!2306 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2307)
!2307 = distinct !DILocation(line: 1498, column: 37, scope: !2291)
!2308 = !DILocation(line: 1498, column: 33, scope: !2291)
!2309 = !DILocation(line: 1498, column: 17, scope: !2291)
!2310 = !DILocation(line: 1498, column: 14, scope: !2291)
!2311 = !DILocation(line: 1498, column: 72, scope: !2291)
!2312 = !DILocation(line: 1499, column: 24, scope: !2291)
!2313 = !DILocation(line: 1499, column: 14, scope: !2291)
!2314 = !DILocation(line: 1499, column: 35, scope: !2291)
!2315 = !DILocation(line: 1501, column: 2, scope: !2281)
!2316 = distinct !DISubprogram(name: "fd_write", linkageName: "_ZN4wasi13lib_generated8fd_write17h4038c76fd62d03c7E", scope: !351, file: !588, line: 1694, type: !2317, scopeLine: 1694, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2328)
!2317 = !DISubroutineType(types: !2318)
!2318 = !{!591, !44, !2319}
!2319 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[wasi::lib_generated::Ciovec]", file: !2, size: 64, align: 32, elements: !2320, templateParams: !13, identifier: "33c0c2eb9a734cc3770653ca55b8aa97")
!2320 = !{!2321, !2327}
!2321 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !2319, file: !2, baseType: !2322, size: 32, align: 32)
!2322 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !2323, size: 32, align: 32, dwarfAddressSpace: 0)
!2323 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ciovec", scope: !351, file: !2, size: 64, align: 32, flags: DIFlagPublic, elements: !2324, templateParams: !13, identifier: "b2077d80f9161ef7f3db89fa8e469af8")
!2324 = !{!2325, !2326}
!2325 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !2323, file: !2, baseType: !847, size: 32, align: 32, flags: DIFlagPublic)
!2326 = !DIDerivedType(tag: DW_TAG_member, name: "buf_len", scope: !2323, file: !2, baseType: !9, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
!2327 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !2319, file: !2, baseType: !9, size: 32, align: 32, offset: 32)
!2328 = !{!2329, !2330, !2331, !2333}
!2329 = !DILocalVariable(name: "fd", arg: 1, scope: !2316, file: !588, line: 1694, type: !44)
!2330 = !DILocalVariable(name: "iovs", arg: 2, scope: !2316, file: !588, line: 1694, type: !2319)
!2331 = !DILocalVariable(name: "rp0", scope: !2332, file: !588, line: 1695, type: !618, align: 32)
!2332 = distinct !DILexicalBlock(scope: !2316, file: !588, line: 1695, column: 5)
!2333 = !DILocalVariable(name: "ret", scope: !2334, file: !588, line: 1696, type: !631, align: 32)
!2334 = distinct !DILexicalBlock(scope: !2332, file: !588, line: 1696, column: 5)
!2335 = !DILocation(line: 1694, column: 24, scope: !2316)
!2336 = !DILocation(line: 1694, column: 32, scope: !2316)
!2337 = !DILocation(line: 1695, column: 9, scope: !2332)
!2338 = !DILocation(line: 1695, column: 19, scope: !2316)
!2339 = !DILocalVariable(name: "self", arg: 1, scope: !2340, file: !2098, line: 724, type: !2319)
!2340 = distinct !DISubprogram(name: "as_ptr<wasi::lib_generated::Ciovec>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h61c7005b39ae494aE", scope: !2099, file: !2098, line: 724, type: !2341, scopeLine: 724, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !2345, retainedNodes: !2344)
!2341 = !DISubroutineType(types: !2342)
!2342 = !{!2343, !2319}
!2343 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const wasi::lib_generated::Ciovec", baseType: !2323, size: 32, align: 32, dwarfAddressSpace: 0)
!2344 = !{!2339}
!2345 = !{!2346}
!2346 = !DITemplateTypeParameter(name: "T", type: !2323)
!2347 = !DILocation(line: 724, column: 25, scope: !2340, inlinedAt: !2348)
!2348 = distinct !DILocation(line: 1698, column: 14, scope: !2332)
!2349 = !DILocation(line: 1698, column: 9, scope: !2332)
!2350 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2351)
!2351 = distinct !DILocation(line: 1700, column: 13, scope: !2332)
!2352 = !DILocation(line: 1700, column: 9, scope: !2332)
!2353 = !DILocation(line: 1696, column: 15, scope: !2332)
!2354 = !DILocation(line: 1696, column: 9, scope: !2334)
!2355 = !DILocation(line: 1702, column: 5, scope: !2334)
!2356 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2357)
!2357 = distinct !DILocation(line: 1703, column: 37, scope: !2334)
!2358 = !DILocation(line: 1703, column: 33, scope: !2334)
!2359 = !DILocation(line: 1703, column: 17, scope: !2334)
!2360 = !DILocation(line: 1703, column: 14, scope: !2334)
!2361 = !DILocation(line: 1703, column: 72, scope: !2334)
!2362 = !DILocation(line: 1704, column: 24, scope: !2334)
!2363 = !DILocation(line: 1704, column: 14, scope: !2334)
!2364 = !DILocation(line: 1704, column: 35, scope: !2334)
!2365 = !DILocation(line: 1706, column: 2, scope: !2316)
!2366 = distinct !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated9Eventtype3raw17hf39dfad6b7758084E", scope: !901, file: !588, line: 747, type: !2367, scopeLine: 747, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2370, retainedNodes: !2371)
!2367 = !DISubroutineType(types: !2368)
!2368 = !{!26, !2369}
!2369 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&wasi::lib_generated::Eventtype", baseType: !901, size: 32, align: 32, dwarfAddressSpace: 0)
!2370 = !DISubprogram(name: "raw", linkageName: "_ZN4wasi13lib_generated9Eventtype3raw17hf39dfad6b7758084E", scope: !901, file: !588, line: 747, type: !2367, scopeLine: 747, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2371 = !{!2372}
!2372 = !DILocalVariable(name: "self", arg: 1, scope: !2366, file: !588, line: 747, type: !2369)
!2373 = !DILocation(line: 747, column: 22, scope: !2366)
!2374 = !DILocation(line: 748, column: 9, scope: !2366)
!2375 = !DILocation(line: 749, column: 6, scope: !2366)
!2376 = distinct !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated9Eventtype4name17h6a88a0e670107d50E", scope: !901, file: !588, line: 751, type: !2377, scopeLine: 751, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2379, retainedNodes: !2380)
!2377 = !DISubroutineType(types: !2378)
!2378 = !{!22, !2369}
!2379 = !DISubprogram(name: "name", linkageName: "_ZN4wasi13lib_generated9Eventtype4name17h6a88a0e670107d50E", scope: !901, file: !588, line: 751, type: !2377, scopeLine: 751, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2380 = !{!2381}
!2381 = !DILocalVariable(name: "self", arg: 1, scope: !2376, file: !588, line: 751, type: !2369)
!2382 = !DILocation(line: 751, column: 17, scope: !2376)
!2383 = !DILocation(line: 752, column: 9, scope: !2376)
!2384 = !DILocation(line: 756, column: 27, scope: !2376)
!2385 = !DILocation(line: 753, column: 18, scope: !2376)
!2386 = !DILocation(line: 754, column: 18, scope: !2376)
!2387 = !DILocation(line: 755, column: 18, scope: !2376)
!2388 = !DILocation(line: 758, column: 6, scope: !2376)
!2389 = distinct !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated9Eventtype7message17h94566f151fe315c9E", scope: !901, file: !588, line: 759, type: !2377, scopeLine: 759, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, declaration: !2390, retainedNodes: !2391)
!2390 = !DISubprogram(name: "message", linkageName: "_ZN4wasi13lib_generated9Eventtype7message17h94566f151fe315c9E", scope: !901, file: !588, line: 759, type: !2377, scopeLine: 759, flags: DIFlagPrototyped, spFlags: 0, templateParams: !13)
!2391 = !{!2392}
!2392 = !DILocalVariable(name: "self", arg: 1, scope: !2389, file: !588, line: 759, type: !2369)
!2393 = !DILocation(line: 759, column: 20, scope: !2389)
!2394 = !DILocation(line: 760, column: 9, scope: !2389)
!2395 = !DILocation(line: 773, column: 27, scope: !2389)
!2396 = !DILocation(line: 762, column: 17, scope: !2389)
!2397 = !DILocation(line: 766, column: 17, scope: !2389)
!2398 = !DILocation(line: 770, column: 17, scope: !2389)
!2399 = !DILocation(line: 775, column: 6, scope: !2389)
!2400 = distinct !DISubprogram(name: "fd_advise", linkageName: "_ZN4wasi13lib_generated9fd_advise17h93e0fb2b560d0ca7E", scope: !351, file: !588, line: 1323, type: !2401, scopeLine: 1323, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2403)
!2401 = !DISubroutineType(types: !2402)
!2402 = !{!667, !44, !360, !360, !1868}
!2403 = !{!2404, !2405, !2406, !2407, !2408}
!2404 = !DILocalVariable(name: "fd", arg: 1, scope: !2400, file: !588, line: 1324, type: !44)
!2405 = !DILocalVariable(name: "offset", arg: 2, scope: !2400, file: !588, line: 1325, type: !360)
!2406 = !DILocalVariable(name: "len", arg: 3, scope: !2400, file: !588, line: 1326, type: !360)
!2407 = !DILocalVariable(name: "advice", arg: 4, scope: !2400, file: !588, line: 1327, type: !1868)
!2408 = !DILocalVariable(name: "ret", scope: !2409, file: !588, line: 1329, type: !631, align: 32)
!2409 = distinct !DILexicalBlock(scope: !2400, file: !588, line: 1329, column: 5)
!2410 = !DILocation(line: 1324, column: 5, scope: !2400)
!2411 = !DILocation(line: 1325, column: 5, scope: !2400)
!2412 = !DILocation(line: 1326, column: 5, scope: !2400)
!2413 = !DILocation(line: 1327, column: 5, scope: !2400)
!2414 = !DILocation(line: 1330, column: 81, scope: !2400)
!2415 = !DILocation(line: 1330, column: 9, scope: !2400)
!2416 = !DILocation(line: 1329, column: 9, scope: !2409)
!2417 = !DILocation(line: 1331, column: 5, scope: !2409)
!2418 = !DILocation(line: 1332, column: 14, scope: !2409)
!2419 = !DILocation(line: 1332, column: 19, scope: !2409)
!2420 = !DILocation(line: 1333, column: 24, scope: !2409)
!2421 = !DILocation(line: 1333, column: 14, scope: !2409)
!2422 = !DILocation(line: 1333, column: 35, scope: !2409)
!2423 = !DILocation(line: 1335, column: 2, scope: !2400)
!2424 = distinct !DISubprogram(name: "fd_pwrite", linkageName: "_ZN4wasi13lib_generated9fd_pwrite17ha5ca9f0b139d9c68E", scope: !351, file: !588, line: 1541, type: !2425, scopeLine: 1541, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2427)
!2425 = !DISubroutineType(types: !2426)
!2426 = !{!591, !44, !2319, !360}
!2427 = !{!2428, !2429, !2430, !2431, !2433}
!2428 = !DILocalVariable(name: "fd", arg: 1, scope: !2424, file: !588, line: 1541, type: !44)
!2429 = !DILocalVariable(name: "iovs", arg: 2, scope: !2424, file: !588, line: 1541, type: !2319)
!2430 = !DILocalVariable(name: "offset", arg: 3, scope: !2424, file: !588, line: 1541, type: !360)
!2431 = !DILocalVariable(name: "rp0", scope: !2432, file: !588, line: 1542, type: !618, align: 32)
!2432 = distinct !DILexicalBlock(scope: !2424, file: !588, line: 1542, column: 5)
!2433 = !DILocalVariable(name: "ret", scope: !2434, file: !588, line: 1543, type: !631, align: 32)
!2434 = distinct !DILexicalBlock(scope: !2432, file: !588, line: 1543, column: 5)
!2435 = !DILocation(line: 1541, column: 25, scope: !2424)
!2436 = !DILocation(line: 1541, column: 33, scope: !2424)
!2437 = !DILocation(line: 1541, column: 56, scope: !2424)
!2438 = !DILocation(line: 1542, column: 9, scope: !2432)
!2439 = !DILocation(line: 1542, column: 19, scope: !2424)
!2440 = !DILocation(line: 724, column: 25, scope: !2340, inlinedAt: !2441)
!2441 = distinct !DILocation(line: 1545, column: 14, scope: !2432)
!2442 = !DILocation(line: 1545, column: 9, scope: !2432)
!2443 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2444)
!2444 = distinct !DILocation(line: 1548, column: 13, scope: !2432)
!2445 = !DILocation(line: 1548, column: 9, scope: !2432)
!2446 = !DILocation(line: 1543, column: 15, scope: !2432)
!2447 = !DILocation(line: 1543, column: 9, scope: !2434)
!2448 = !DILocation(line: 1550, column: 5, scope: !2434)
!2449 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2450)
!2450 = distinct !DILocation(line: 1551, column: 37, scope: !2434)
!2451 = !DILocation(line: 1551, column: 33, scope: !2434)
!2452 = !DILocation(line: 1551, column: 17, scope: !2434)
!2453 = !DILocation(line: 1551, column: 14, scope: !2434)
!2454 = !DILocation(line: 1551, column: 72, scope: !2434)
!2455 = !DILocation(line: 1552, column: 24, scope: !2434)
!2456 = !DILocation(line: 1552, column: 14, scope: !2434)
!2457 = !DILocation(line: 1552, column: 35, scope: !2434)
!2458 = !DILocation(line: 1554, column: 2, scope: !2424)
!2459 = distinct !DISubprogram(name: "path_link", linkageName: "_ZN4wasi13lib_generated9path_link17h2269c99aa733f901E", scope: !351, file: !588, line: 1794, type: !2460, scopeLine: 1794, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2462)
!2460 = !DISubroutineType(types: !2461)
!2461 = !{!667, !44, !44, !22, !44, !22}
!2462 = !{!2463, !2464, !2465, !2466, !2467, !2468}
!2463 = !DILocalVariable(name: "old_fd", arg: 1, scope: !2459, file: !588, line: 1795, type: !44)
!2464 = !DILocalVariable(name: "old_flags", arg: 2, scope: !2459, file: !588, line: 1796, type: !44)
!2465 = !DILocalVariable(name: "old_path", arg: 3, scope: !2459, file: !588, line: 1797, type: !22)
!2466 = !DILocalVariable(name: "new_fd", arg: 4, scope: !2459, file: !588, line: 1798, type: !44)
!2467 = !DILocalVariable(name: "new_path", arg: 5, scope: !2459, file: !588, line: 1799, type: !22)
!2468 = !DILocalVariable(name: "ret", scope: !2469, file: !588, line: 1801, type: !631, align: 32)
!2469 = distinct !DILexicalBlock(scope: !2459, file: !588, line: 1801, column: 5)
!2470 = !DILocation(line: 1795, column: 5, scope: !2459)
!2471 = !DILocation(line: 1796, column: 5, scope: !2459)
!2472 = !DILocation(line: 1797, column: 5, scope: !2459)
!2473 = !DILocation(line: 1798, column: 5, scope: !2459)
!2474 = !DILocation(line: 1799, column: 5, scope: !2459)
!2475 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !2476)
!2476 = distinct !DILocation(line: 1804, column: 18, scope: !2459)
!2477 = !DILocation(line: 1804, column: 9, scope: !2459)
!2478 = !DILocation(line: 1805, column: 18, scope: !2459)
!2479 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !2480)
!2480 = distinct !DILocation(line: 1807, column: 18, scope: !2459)
!2481 = !DILocation(line: 1807, column: 9, scope: !2459)
!2482 = !DILocation(line: 1808, column: 18, scope: !2459)
!2483 = !DILocation(line: 1801, column: 15, scope: !2459)
!2484 = !DILocation(line: 1801, column: 9, scope: !2469)
!2485 = !DILocation(line: 1810, column: 5, scope: !2469)
!2486 = !DILocation(line: 1811, column: 14, scope: !2469)
!2487 = !DILocation(line: 1811, column: 19, scope: !2469)
!2488 = !DILocation(line: 1812, column: 24, scope: !2469)
!2489 = !DILocation(line: 1812, column: 14, scope: !2469)
!2490 = !DILocation(line: 1812, column: 35, scope: !2469)
!2491 = !DILocation(line: 1814, column: 2, scope: !2459)
!2492 = distinct !DISubprogram(name: "path_open", linkageName: "_ZN4wasi13lib_generated9path_open17hffe6bc4ef505fe1eE", scope: !351, file: !588, line: 1841, type: !2493, scopeLine: 1841, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2495)
!2493 = !DISubroutineType(types: !2494)
!2494 = !{!958, !44, !44, !22, !12, !360, !360, !12}
!2495 = !{!2496, !2497, !2498, !2499, !2500, !2501, !2502, !2503, !2505}
!2496 = !DILocalVariable(name: "fd", arg: 1, scope: !2492, file: !588, line: 1842, type: !44)
!2497 = !DILocalVariable(name: "dirflags", arg: 2, scope: !2492, file: !588, line: 1843, type: !44)
!2498 = !DILocalVariable(name: "path", arg: 3, scope: !2492, file: !588, line: 1844, type: !22)
!2499 = !DILocalVariable(name: "oflags", arg: 4, scope: !2492, file: !588, line: 1845, type: !12)
!2500 = !DILocalVariable(name: "fs_rights_base", arg: 5, scope: !2492, file: !588, line: 1846, type: !360)
!2501 = !DILocalVariable(name: "fs_rights_inheriting", arg: 6, scope: !2492, file: !588, line: 1847, type: !360)
!2502 = !DILocalVariable(name: "fdflags", arg: 7, scope: !2492, file: !588, line: 1848, type: !12)
!2503 = !DILocalVariable(name: "rp0", scope: !2504, file: !588, line: 1850, type: !977, align: 32)
!2504 = distinct !DILexicalBlock(scope: !2492, file: !588, line: 1850, column: 5)
!2505 = !DILocalVariable(name: "ret", scope: !2506, file: !588, line: 1851, type: !631, align: 32)
!2506 = distinct !DILexicalBlock(scope: !2504, file: !588, line: 1851, column: 5)
!2507 = !DILocation(line: 1842, column: 5, scope: !2492)
!2508 = !DILocation(line: 1843, column: 5, scope: !2492)
!2509 = !DILocation(line: 1844, column: 5, scope: !2492)
!2510 = !DILocation(line: 1845, column: 5, scope: !2492)
!2511 = !DILocation(line: 1846, column: 5, scope: !2492)
!2512 = !DILocation(line: 1847, column: 5, scope: !2492)
!2513 = !DILocation(line: 1848, column: 5, scope: !2492)
!2514 = !DILocation(line: 1850, column: 9, scope: !2504)
!2515 = !DILocation(line: 1850, column: 19, scope: !2492)
!2516 = !DILocation(line: 562, column: 25, scope: !844, inlinedAt: !2517)
!2517 = distinct !DILocation(line: 1854, column: 14, scope: !2504)
!2518 = !DILocation(line: 1854, column: 9, scope: !2504)
!2519 = !DILocation(line: 1855, column: 14, scope: !2504)
!2520 = !DILocation(line: 1856, column: 9, scope: !2504)
!2521 = !DILocation(line: 1859, column: 9, scope: !2504)
!2522 = !DILocation(line: 560, column: 29, scope: !992, inlinedAt: !2523)
!2523 = distinct !DILocation(line: 1860, column: 13, scope: !2504)
!2524 = !DILocation(line: 1860, column: 9, scope: !2504)
!2525 = !DILocation(line: 1851, column: 15, scope: !2504)
!2526 = !DILocation(line: 1851, column: 9, scope: !2506)
!2527 = !DILocation(line: 1862, column: 5, scope: !2506)
!2528 = !DILocation(line: 560, column: 29, scope: !992, inlinedAt: !2529)
!2529 = distinct !DILocation(line: 1863, column: 37, scope: !2506)
!2530 = !DILocation(line: 1863, column: 33, scope: !2506)
!2531 = !DILocation(line: 1863, column: 17, scope: !2506)
!2532 = !DILocation(line: 1863, column: 14, scope: !2506)
!2533 = !DILocation(line: 1863, column: 70, scope: !2506)
!2534 = !DILocation(line: 1864, column: 24, scope: !2506)
!2535 = !DILocation(line: 1864, column: 14, scope: !2506)
!2536 = !DILocation(line: 1864, column: 35, scope: !2506)
!2537 = !DILocation(line: 1866, column: 2, scope: !2492)
!2538 = distinct !DISubprogram(name: "proc_exit", linkageName: "_ZN4wasi13lib_generated9proc_exit17h6ac6fed0d1947302E", scope: !351, file: !588, line: 2018, type: !2539, scopeLine: 2018, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2541)
!2539 = !DISubroutineType(types: !2540)
!2540 = !{null, !44}
!2541 = !{!2542}
!2542 = !DILocalVariable(name: "rval", arg: 1, scope: !2538, file: !588, line: 2018, type: !44)
!2543 = !DILocation(line: 2018, column: 25, scope: !2538)
!2544 = !DILocation(line: 2019, column: 5, scope: !2538)
!2545 = distinct !DISubprogram(name: "sock_recv", linkageName: "_ZN4wasi13lib_generated9sock_recv17h8f741338cb57a0c4E", scope: !351, file: !588, line: 2096, type: !2546, scopeLine: 2096, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2567)
!2546 = !DISubroutineType(types: !2547)
!2547 = !{!2548, !44, !2076, !12}
!2548 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(usize, u16), wasi::lib_generated::Errno>", scope: !60, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !2549, templateParams: !13, identifier: "6ebb93d9e6f7ad14ab5a33ea92921ae1")
!2549 = !{!2550}
!2550 = !DICompositeType(tag: DW_TAG_variant_part, scope: !2548, file: !2, size: 96, align: 32, elements: !2551, templateParams: !13, identifier: "53945f5ac807f70aeb077851ec8b7307", discriminator: !2566)
!2551 = !{!2552, !2562}
!2552 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !2550, file: !2, baseType: !2553, size: 96, align: 32, extraData: i16 0)
!2553 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !2548, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !2554, templateParams: !2560, identifier: "f48f982618caf826bda7cead3ec937e6")
!2554 = !{!2555}
!2555 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2553, file: !2, baseType: !2556, size: 64, align: 32, offset: 32, flags: DIFlagPublic)
!2556 = !DICompositeType(tag: DW_TAG_structure_type, name: "(usize, u16)", file: !2, size: 64, align: 32, elements: !2557, templateParams: !13, identifier: "149a26d47b12ca32f750131f209beb2f")
!2557 = !{!2558, !2559}
!2558 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2556, file: !2, baseType: !9, size: 32, align: 32)
!2559 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !2556, file: !2, baseType: !12, size: 16, align: 16, offset: 32)
!2560 = !{!2561, !601}
!2561 = !DITemplateTypeParameter(name: "T", type: !2556)
!2562 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !2550, file: !2, baseType: !2563, size: 96, align: 32, extraData: i16 1)
!2563 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !2548, file: !2, size: 96, align: 32, flags: DIFlagPublic, elements: !2564, templateParams: !2560, identifier: "578b60c41202f4a28610deed5602684e")
!2564 = !{!2565}
!2565 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2563, file: !2, baseType: !602, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
!2566 = !DIDerivedType(tag: DW_TAG_member, scope: !2548, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagArtificial)
!2567 = !{!2568, !2569, !2570, !2571, !2573, !2582}
!2568 = !DILocalVariable(name: "fd", arg: 1, scope: !2545, file: !588, line: 2097, type: !44)
!2569 = !DILocalVariable(name: "ri_data", arg: 2, scope: !2545, file: !588, line: 2098, type: !2076)
!2570 = !DILocalVariable(name: "ri_flags", arg: 3, scope: !2545, file: !588, line: 2099, type: !12)
!2571 = !DILocalVariable(name: "rp0", scope: !2572, file: !588, line: 2101, type: !618, align: 32)
!2572 = distinct !DILexicalBlock(scope: !2545, file: !588, line: 2101, column: 5)
!2573 = !DILocalVariable(name: "rp1", scope: !2574, file: !588, line: 2102, type: !2575, align: 16)
!2574 = distinct !DILexicalBlock(scope: !2572, file: !588, line: 2102, column: 5)
!2575 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<u16>", scope: !619, file: !2, size: 16, align: 16, elements: !2576, templateParams: !338, identifier: "94fee1ce31e4a1c95a23897d13b43851")
!2576 = !{!2577, !2578}
!2577 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !2575, file: !2, baseType: !7, align: 8)
!2578 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !2575, file: !2, baseType: !2579, size: 16, align: 16)
!2579 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<u16>", scope: !625, file: !2, size: 16, align: 16, flags: DIFlagPublic, elements: !2580, templateParams: !338, identifier: "f75fee542e0991c560647a24f147f6b6")
!2580 = !{!2581}
!2581 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !2579, file: !2, baseType: !12, size: 16, align: 16, flags: DIFlagPrivate)
!2582 = !DILocalVariable(name: "ret", scope: !2583, file: !588, line: 2103, type: !631, align: 32)
!2583 = distinct !DILexicalBlock(scope: !2574, file: !588, line: 2103, column: 5)
!2584 = !DILocation(line: 2097, column: 5, scope: !2545)
!2585 = !DILocation(line: 2098, column: 5, scope: !2545)
!2586 = !DILocation(line: 2099, column: 5, scope: !2545)
!2587 = !DILocation(line: 2101, column: 9, scope: !2572)
!2588 = !DILocation(line: 2102, column: 9, scope: !2574)
!2589 = !DILocation(line: 2101, column: 19, scope: !2545)
!2590 = !DILocation(line: 2102, column: 19, scope: !2572)
!2591 = !DILocation(line: 724, column: 25, scope: !2097, inlinedAt: !2592)
!2592 = distinct !DILocation(line: 2105, column: 17, scope: !2574)
!2593 = !DILocation(line: 2105, column: 9, scope: !2574)
!2594 = !DILocation(line: 2107, column: 9, scope: !2574)
!2595 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2596)
!2596 = distinct !DILocation(line: 2108, column: 13, scope: !2574)
!2597 = !DILocation(line: 2108, column: 9, scope: !2574)
!2598 = !DILocalVariable(name: "self", arg: 1, scope: !2599, file: !641, line: 560, type: !2603)
!2599 = distinct !DISubprogram(name: "as_mut_ptr<u16>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hc152d6c79c998684E", scope: !2575, file: !641, line: 560, type: !2600, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !46, templateParams: !338, declaration: !2604, retainedNodes: !2605)
!2600 = !DISubroutineType(types: !2601)
!2601 = !{!2602, !2603}
!2602 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u16", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!2603 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<u16>", baseType: !2575, size: 32, align: 32, dwarfAddressSpace: 0)
!2604 = !DISubprogram(name: "as_mut_ptr<u16>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17hc152d6c79c998684E", scope: !2575, file: !641, line: 560, type: !2600, scopeLine: 560, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !338)
!2605 = !{!2598}
!2606 = !DILocation(line: 560, column: 29, scope: !2599, inlinedAt: !2607)
!2607 = distinct !DILocation(line: 2109, column: 13, scope: !2574)
!2608 = !DILocation(line: 2109, column: 9, scope: !2574)
!2609 = !DILocation(line: 2103, column: 15, scope: !2574)
!2610 = !DILocation(line: 2103, column: 9, scope: !2583)
!2611 = !DILocation(line: 2111, column: 5, scope: !2583)
!2612 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2613)
!2613 = distinct !DILocation(line: 2113, column: 33, scope: !2583)
!2614 = !DILocation(line: 2113, column: 29, scope: !2583)
!2615 = !DILocation(line: 2113, column: 13, scope: !2583)
!2616 = !DILocation(line: 560, column: 29, scope: !2599, inlinedAt: !2617)
!2617 = distinct !DILocation(line: 2114, column: 33, scope: !2583)
!2618 = !DILocation(line: 2114, column: 29, scope: !2583)
!2619 = !DILocation(line: 2114, column: 13, scope: !2583)
!2620 = !DILocation(line: 2112, column: 14, scope: !2583)
!2621 = !DILocation(line: 2115, column: 10, scope: !2583)
!2622 = !DILocation(line: 2116, column: 24, scope: !2583)
!2623 = !DILocation(line: 2116, column: 14, scope: !2583)
!2624 = !DILocation(line: 2116, column: 35, scope: !2583)
!2625 = !DILocation(line: 2118, column: 2, scope: !2545)
!2626 = distinct !DISubprogram(name: "sock_send", linkageName: "_ZN4wasi13lib_generated9sock_send17h88a407874ec40b2fE", scope: !351, file: !588, line: 2132, type: !2627, scopeLine: 2132, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2629)
!2627 = !DISubroutineType(types: !2628)
!2628 = !{!591, !44, !2319, !12}
!2629 = !{!2630, !2631, !2632, !2633, !2635}
!2630 = !DILocalVariable(name: "fd", arg: 1, scope: !2626, file: !588, line: 2133, type: !44)
!2631 = !DILocalVariable(name: "si_data", arg: 2, scope: !2626, file: !588, line: 2134, type: !2319)
!2632 = !DILocalVariable(name: "si_flags", arg: 3, scope: !2626, file: !588, line: 2135, type: !12)
!2633 = !DILocalVariable(name: "rp0", scope: !2634, file: !588, line: 2137, type: !618, align: 32)
!2634 = distinct !DILexicalBlock(scope: !2626, file: !588, line: 2137, column: 5)
!2635 = !DILocalVariable(name: "ret", scope: !2636, file: !588, line: 2138, type: !631, align: 32)
!2636 = distinct !DILexicalBlock(scope: !2634, file: !588, line: 2138, column: 5)
!2637 = !DILocation(line: 2133, column: 5, scope: !2626)
!2638 = !DILocation(line: 2134, column: 5, scope: !2626)
!2639 = !DILocation(line: 2135, column: 5, scope: !2626)
!2640 = !DILocation(line: 2137, column: 9, scope: !2634)
!2641 = !DILocation(line: 2137, column: 19, scope: !2626)
!2642 = !DILocation(line: 724, column: 25, scope: !2340, inlinedAt: !2643)
!2643 = distinct !DILocation(line: 2140, column: 17, scope: !2634)
!2644 = !DILocation(line: 2140, column: 9, scope: !2634)
!2645 = !DILocation(line: 2142, column: 9, scope: !2634)
!2646 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2647)
!2647 = distinct !DILocation(line: 2143, column: 13, scope: !2634)
!2648 = !DILocation(line: 2143, column: 9, scope: !2634)
!2649 = !DILocation(line: 2138, column: 15, scope: !2634)
!2650 = !DILocation(line: 2138, column: 9, scope: !2636)
!2651 = !DILocation(line: 2145, column: 5, scope: !2636)
!2652 = !DILocation(line: 560, column: 29, scope: !640, inlinedAt: !2653)
!2653 = distinct !DILocation(line: 2146, column: 37, scope: !2636)
!2654 = !DILocation(line: 2146, column: 33, scope: !2636)
!2655 = !DILocation(line: 2146, column: 17, scope: !2636)
!2656 = !DILocation(line: 2146, column: 14, scope: !2636)
!2657 = !DILocation(line: 2146, column: 72, scope: !2636)
!2658 = !DILocation(line: 2147, column: 24, scope: !2636)
!2659 = !DILocation(line: 2147, column: 14, scope: !2636)
!2660 = !DILocation(line: 2147, column: 35, scope: !2636)
!2661 = !DILocation(line: 2149, column: 2, scope: !2626)
!2662 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN63_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Debug$GT$3fmt17hefc646a6f106b0e7E", scope: !2663, file: !588, line: 395, type: !2664, scopeLine: 395, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2666)
!2663 = !DINamespace(name: "{impl#3}", scope: !351)
!2664 = !DISubroutineType(types: !2665)
!2665 = !{!59, !1688, !78}
!2666 = !{!2667, !2668}
!2667 = !DILocalVariable(name: "self", arg: 1, scope: !2662, file: !588, line: 395, type: !1688)
!2668 = !DILocalVariable(name: "f", arg: 2, scope: !2662, file: !588, line: 395, type: !78)
!2669 = !DILocation(line: 395, column: 12, scope: !2662)
!2670 = !DILocation(line: 395, column: 19, scope: !2662)
!2671 = !DILocation(line: 396, column: 11, scope: !2662)
!2672 = !DILocation(line: 397, column: 14, scope: !2662)
!2673 = !DILocation(line: 398, column: 34, scope: !2662)
!2674 = !DILocation(line: 398, column: 14, scope: !2662)
!2675 = !DILocation(line: 399, column: 37, scope: !2662)
!2676 = !DILocation(line: 399, column: 14, scope: !2662)
!2677 = !DILocation(line: 400, column: 14, scope: !2662)
!2678 = !DILocation(line: 401, column: 6, scope: !2662)
!2679 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN64_$LT$wasi..lib_generated..Advice$u20$as$u20$core..fmt..Debug$GT$3fmt17h269077da97230a8cE", scope: !2680, file: !588, line: 651, type: !2681, scopeLine: 651, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2683)
!2680 = !DINamespace(name: "{impl#10}", scope: !351)
!2681 = !DISubroutineType(types: !2682)
!2682 = !{!59, !1873, !78}
!2683 = !{!2684, !2685}
!2684 = !DILocalVariable(name: "self", arg: 1, scope: !2679, file: !588, line: 651, type: !1873)
!2685 = !DILocalVariable(name: "f", arg: 2, scope: !2679, file: !588, line: 651, type: !78)
!2686 = !DILocation(line: 651, column: 12, scope: !2679)
!2687 = !DILocation(line: 651, column: 19, scope: !2679)
!2688 = !DILocation(line: 652, column: 11, scope: !2679)
!2689 = !DILocation(line: 653, column: 14, scope: !2679)
!2690 = !DILocation(line: 654, column: 34, scope: !2679)
!2691 = !DILocation(line: 654, column: 14, scope: !2679)
!2692 = !DILocation(line: 655, column: 37, scope: !2679)
!2693 = !DILocation(line: 655, column: 14, scope: !2679)
!2694 = !DILocation(line: 656, column: 14, scope: !2679)
!2695 = !DILocation(line: 657, column: 6, scope: !2679)
!2696 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN64_$LT$wasi..lib_generated..Signal$u20$as$u20$core..fmt..Debug$GT$3fmt17ha37f77817edb24beE", scope: !2697, file: !588, line: 1131, type: !2698, scopeLine: 1131, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2700)
!2697 = !DINamespace(name: "{impl#14}", scope: !351)
!2698 = !DISubroutineType(types: !2699)
!2699 = !{!59, !1913, !78}
!2700 = !{!2701, !2702}
!2701 = !DILocalVariable(name: "self", arg: 1, scope: !2696, file: !588, line: 1131, type: !1913)
!2702 = !DILocalVariable(name: "f", arg: 2, scope: !2696, file: !588, line: 1131, type: !78)
!2703 = !DILocation(line: 1131, column: 12, scope: !2696)
!2704 = !DILocation(line: 1131, column: 19, scope: !2696)
!2705 = !DILocation(line: 1132, column: 11, scope: !2696)
!2706 = !DILocation(line: 1133, column: 14, scope: !2696)
!2707 = !DILocation(line: 1134, column: 34, scope: !2696)
!2708 = !DILocation(line: 1134, column: 14, scope: !2696)
!2709 = !DILocation(line: 1135, column: 37, scope: !2696)
!2710 = !DILocation(line: 1135, column: 14, scope: !2696)
!2711 = !DILocation(line: 1136, column: 14, scope: !2696)
!2712 = !DILocation(line: 1137, column: 6, scope: !2696)
!2713 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN64_$LT$wasi..lib_generated..Whence$u20$as$u20$core..fmt..Debug$GT$3fmt17h2c6ea1883e0a7c8eE", scope: !2714, file: !588, line: 540, type: !2715, scopeLine: 540, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2717)
!2714 = !DINamespace(name: "{impl#6}", scope: !351)
!2715 = !DISubroutineType(types: !2716)
!2716 = !{!59, !2006, !78}
!2717 = !{!2718, !2719}
!2718 = !DILocalVariable(name: "self", arg: 1, scope: !2713, file: !588, line: 540, type: !2006)
!2719 = !DILocalVariable(name: "f", arg: 2, scope: !2713, file: !588, line: 540, type: !78)
!2720 = !DILocation(line: 540, column: 12, scope: !2713)
!2721 = !DILocation(line: 540, column: 19, scope: !2713)
!2722 = !DILocation(line: 541, column: 11, scope: !2713)
!2723 = !DILocation(line: 542, column: 14, scope: !2713)
!2724 = !DILocation(line: 543, column: 34, scope: !2713)
!2725 = !DILocation(line: 543, column: 14, scope: !2713)
!2726 = !DILocation(line: 544, column: 37, scope: !2713)
!2727 = !DILocation(line: 544, column: 14, scope: !2713)
!2728 = !DILocation(line: 545, column: 14, scope: !2713)
!2729 = !DILocation(line: 546, column: 6, scope: !2713)
!2730 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN65_$LT$wasi..lib_generated..Clockid$u20$as$u20$core..fmt..Debug$GT$3fmt17h99a1a83dc5fa5389E", scope: !2731, file: !588, line: 58, type: !2732, scopeLine: 58, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2734)
!2731 = !DINamespace(name: "{impl#1}", scope: !351)
!2732 = !DISubroutineType(types: !2733)
!2733 = !{!59, !2040, !78}
!2734 = !{!2735, !2736}
!2735 = !DILocalVariable(name: "self", arg: 1, scope: !2730, file: !588, line: 58, type: !2040)
!2736 = !DILocalVariable(name: "f", arg: 2, scope: !2730, file: !588, line: 58, type: !78)
!2737 = !DILocation(line: 58, column: 12, scope: !2730)
!2738 = !DILocation(line: 58, column: 19, scope: !2730)
!2739 = !DILocation(line: 59, column: 11, scope: !2730)
!2740 = !DILocation(line: 60, column: 14, scope: !2730)
!2741 = !DILocation(line: 61, column: 34, scope: !2730)
!2742 = !DILocation(line: 61, column: 14, scope: !2730)
!2743 = !DILocation(line: 62, column: 37, scope: !2730)
!2744 = !DILocation(line: 62, column: 14, scope: !2730)
!2745 = !DILocation(line: 63, column: 14, scope: !2730)
!2746 = !DILocation(line: 64, column: 6, scope: !2730)
!2747 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN65_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Display$GT$3fmt17hd6a85332ea0c6f1aE", scope: !2748, file: !588, line: 404, type: !2664, scopeLine: 404, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2749)
!2748 = !DINamespace(name: "{impl#4}", scope: !351)
!2749 = !{!2750, !2751, !2752, !2760}
!2750 = !DILocalVariable(name: "self", arg: 1, scope: !2747, file: !588, line: 404, type: !1688)
!2751 = !DILocalVariable(name: "f", arg: 2, scope: !2747, file: !588, line: 404, type: !78)
!2752 = !DILocalVariable(name: "args", scope: !2753, file: !588, line: 405, type: !2756, align: 32)
!2753 = !DILexicalBlockFile(scope: !2754, file: !588, discriminator: 0)
!2754 = distinct !DILexicalBlock(scope: !2747, file: !2755, line: 612, column: 24)
!2755 = !DIFile(filename: "/Users/namse/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/macros/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "ec901dc314d42500d18c9938d02d4ad0")
!2756 = !DICompositeType(tag: DW_TAG_structure_type, name: "(&&str, &u16)", file: !2, size: 64, align: 32, elements: !2757, templateParams: !13, identifier: "b615b6307082056bc5dd302aadf5a2e6")
!2757 = !{!2758, !2759}
!2758 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !2756, file: !2, baseType: !273, size: 32, align: 32)
!2759 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !2756, file: !2, baseType: !115, size: 32, align: 32, offset: 32)
!2760 = !DILocalVariable(name: "args", scope: !2761, file: !588, line: 405, type: !2763, align: 32)
!2761 = !DILexicalBlockFile(scope: !2762, file: !588, discriminator: 0)
!2762 = distinct !DILexicalBlock(scope: !2754, file: !2755, line: 612, column: 24)
!2763 = !DICompositeType(tag: DW_TAG_array_type, baseType: !204, size: 128, align: 32, elements: !2764)
!2764 = !{!2765}
!2765 = !DISubrange(count: 2, lowerBound: 0)
!2766 = !DILocation(line: 404, column: 12, scope: !2747)
!2767 = !DILocation(line: 404, column: 19, scope: !2747)
!2768 = !DILocation(line: 405, column: 9, scope: !2761)
!2769 = !DILocation(line: 405, column: 41, scope: !2747)
!2770 = !DILocation(line: 405, column: 9, scope: !2747)
!2771 = !DILocation(line: 405, column: 9, scope: !2753)
!2772 = !DILocation(line: 406, column: 6, scope: !2747)
!2773 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN66_$LT$wasi..lib_generated..Filetype$u20$as$u20$core..fmt..Debug$GT$3fmt17hd766654a1d63f04fE", scope: !2774, file: !588, line: 594, type: !2775, scopeLine: 594, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2777)
!2774 = !DINamespace(name: "{impl#8}", scope: !351)
!2775 = !DISubroutineType(types: !2776)
!2776 = !{!59, !2206, !78}
!2777 = !{!2778, !2779}
!2778 = !DILocalVariable(name: "self", arg: 1, scope: !2773, file: !588, line: 594, type: !2206)
!2779 = !DILocalVariable(name: "f", arg: 2, scope: !2773, file: !588, line: 594, type: !78)
!2780 = !DILocation(line: 594, column: 12, scope: !2773)
!2781 = !DILocation(line: 594, column: 19, scope: !2773)
!2782 = !DILocation(line: 595, column: 11, scope: !2773)
!2783 = !DILocation(line: 596, column: 14, scope: !2773)
!2784 = !DILocation(line: 597, column: 34, scope: !2773)
!2785 = !DILocation(line: 597, column: 14, scope: !2773)
!2786 = !DILocation(line: 598, column: 37, scope: !2773)
!2787 = !DILocation(line: 598, column: 14, scope: !2773)
!2788 = !DILocation(line: 599, column: 14, scope: !2773)
!2789 = !DILocation(line: 600, column: 6, scope: !2773)
!2790 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN67_$LT$wasi..lib_generated..Eventtype$u20$as$u20$core..fmt..Debug$GT$3fmt17h1543bb8c0ad4c88aE", scope: !2791, file: !588, line: 778, type: !2792, scopeLine: 778, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2794)
!2791 = !DINamespace(name: "{impl#12}", scope: !351)
!2792 = !DISubroutineType(types: !2793)
!2793 = !{!59, !2369, !78}
!2794 = !{!2795, !2796}
!2795 = !DILocalVariable(name: "self", arg: 1, scope: !2790, file: !588, line: 778, type: !2369)
!2796 = !DILocalVariable(name: "f", arg: 2, scope: !2790, file: !588, line: 778, type: !78)
!2797 = !DILocation(line: 778, column: 12, scope: !2790)
!2798 = !DILocation(line: 778, column: 19, scope: !2790)
!2799 = !DILocation(line: 779, column: 11, scope: !2790)
!2800 = !DILocation(line: 780, column: 14, scope: !2790)
!2801 = !DILocation(line: 781, column: 34, scope: !2790)
!2802 = !DILocation(line: 781, column: 14, scope: !2790)
!2803 = !DILocation(line: 782, column: 37, scope: !2790)
!2804 = !DILocation(line: 782, column: 14, scope: !2790)
!2805 = !DILocation(line: 783, column: 14, scope: !2790)
!2806 = !DILocation(line: 784, column: 6, scope: !2790)
!2807 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN69_$LT$wasi..lib_generated..Preopentype$u20$as$u20$core..fmt..Debug$GT$3fmt17h0d6b43c88f2d34b4E", scope: !2808, file: !588, line: 1181, type: !2809, scopeLine: 1181, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !46, templateParams: !13, retainedNodes: !2811)
!2808 = !DINamespace(name: "{impl#16}", scope: !351)
!2809 = !DISubroutineType(types: !2810)
!2810 = !{!59, !725, !78}
!2811 = !{!2812, !2813}
!2812 = !DILocalVariable(name: "self", arg: 1, scope: !2807, file: !588, line: 1181, type: !725)
!2813 = !DILocalVariable(name: "f", arg: 2, scope: !2807, file: !588, line: 1181, type: !78)
!2814 = !DILocation(line: 1181, column: 12, scope: !2807)
!2815 = !DILocation(line: 1181, column: 19, scope: !2807)
!2816 = !DILocation(line: 1182, column: 11, scope: !2807)
!2817 = !DILocation(line: 1183, column: 14, scope: !2807)
!2818 = !DILocation(line: 1184, column: 34, scope: !2807)
!2819 = !DILocation(line: 1184, column: 14, scope: !2807)
!2820 = !DILocation(line: 1185, column: 37, scope: !2807)
!2821 = !DILocation(line: 1185, column: 14, scope: !2807)
!2822 = !DILocation(line: 1186, column: 14, scope: !2807)
!2823 = !DILocation(line: 1187, column: 6, scope: !2807)
