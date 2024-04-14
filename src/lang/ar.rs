lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "الحالة"),
        ("Your Desktop", "سطح مكتبك"),
        ("desk_tip", "يمكن الوصول لسطح مكتبك بهذا المعرف والرقم السري."),
        ("Password", "كلمة المرور"),
        ("Ready", "جاهز"),
        ("Established", "تم الانشاء"),
        ("connecting_status", "جاري الاتصال بشبكة RustDesk..."),
        ("Enable service", "تفعيل الخدمة"),
        ("Start service", "بدء الخدمة"),
        ("Service is running", "الخدمة تعمل"),
        ("Service is not running", "الخدمة لا تعمل"),
        ("not_ready_status", "غير جاهز. الرجاء التأكد من الاتصال"),
        ("Control Remote Desktop", "التحكم بسطح المكتب البعيد"),
        ("Transfer file", "نقل ملف"),
        ("Connect", "اتصال"),
        ("Recent sessions", "الجلسات الحديثة"),
        ("Address book", "كتاب العناوين"),
        ("Confirmation", "التأكيد"),
        ("TCP tunneling", "نفق TCP"),
        ("Remove", "ازالة"),
        ("Refresh random password", "تحديث كلمة مرور عشوائية"),
        ("Set your own password", "تعيين كلمة مرور خاصة بك"),
        ("Enable keyboard/mouse", "تفعيل لوحة المفاتيح/الفأرة"),
        ("Enable clipboard", "تفعيل الحافظة"),
        ("Enable file transfer", "تفعيل نقل الملفات"),
        ("Enable TCP tunneling", "تفعيل نفق TCP"),
        ("IP Whitelisting", "القائمة البيضاء للـ IP"),
        ("ID/Relay Server", "معرف خادم الوسيط"),
        ("Import server config", "استيراد إعدادات الخادم"),
        ("Export Server Config", "تصدير إعدادات الخادم"),
        ("Import server configuration successfully", "تم استيراد إعدادات الخادم بنجاح"),
        ("Export server configuration successfully", "تم تصدير إعدادات الخادم بنجاح"),
        ("Invalid server configuration", "إعدادات الخادم غير صحيحة"),
        ("Clipboard is empty", "الحافظة فارغة"),
        ("Stop service", "إيقاف الخدمة"),
        ("Change ID", "تغيير المعرف"),
        ("Your new ID", "معرفك الجديد"),
        ("length %min% to %max%", "الطول من %min% الى %max%"),
        ("starts with a letter", "يبدأ بحرف"),
        ("allowed characters", "الحروف المسموح بها"),
        ("id_change_tip", "فقط a-z, A-Z, 0-9 و _ مسموح بها. اول حرف يجب ان يكون a-z او A-Z. الطول بين 6 و 16."),
        ("Website", "الموقع"),
        ("About", "عن"),
        ("Slogan_tip", "صنع بحب في هذا العالم الفوضوي!"),
        ("Privacy Statement", "بيان الخصوصية"),
        ("Mute", "كتم"),
        ("Build Date", "تاريخ البناء"),
        ("Version", "النسخة"),
        ("Home", "المنزل"),
        ("Audio Input", "مصدر الصوت"),
        ("Enhancements", "التحسينات"),
        ("Hardware Codec", "ترميز العتاد"),
        ("Adaptive bitrate", "معدل بت متكيف"),
        ("ID Server", "معرف الخادم"),
        ("Relay Server", "خادم الوسيط"),
        ("API Server", "خادم API"),
        ("invalid_http", "يجب ان يبدأ بـ http:// او https://"),
        ("Invalid IP", "عنوان IP غير صحيح"),
        ("Invalid format", "صيغة غير صحيحة"),
        ("server_not_support", "الخادم غير مدعوم"),
        ("Not available", "غير متوفر"),
        ("Too frequent", "متكرر جدا"),
        ("Cancel", "إلغاء الأمر"),
        ("Skip", "تجاوز"),
        ("Close", "إغلاق"),
        ("Retry", "إعادة المحاولة"),
        ("OK", "موافق"),
        ("Password Required", "كلمة المرور اجبارية"),
        ("Please enter your password", "الرجاء كتابة كلمة المرور"),
        ("Remember password", "تذكر كلمة المرور"),
        ("Wrong Password", "كلمة مرور خاطئة"),
        ("Do you want to enter again?", "هل تريد الادخال مرة اخرى؟"),
        ("Connection Error", "خطأ غي الاتصال"),
        ("Error", "خطأ"),
        ("Reset by the peer", "تمت اعادة التعيين بواسطة القرين"),
        ("Connecting...", "جاري الاتصال..."),
        ("Connection in progress. Please wait.", "جاري الاتصال, الرجاء الانتظار..."),
        ("Please try 1 minute later", "الرجاء المحاولة بعد دقيقة واحدة"),
        ("Login Error", "خطأ في تسجيل الدخول"),
        ("Successful", "نجاح"),
        ("Connected, waiting for image...", "متصل, جاري انتظار الصورة..."),
        ("Name", "الاسم"),
        ("Type", "النوع"),
        ("Modified", "آخر تعديل"),
        ("Size", "الحجم"),
        ("Show Hidden Files", "عرض الملفات المخفية"),
        ("Receive", "استقبال"),
        ("Send", "ارسال"),
        ("Refresh File", "تحديث الملف"),
        ("Local", "المحلي"),
        ("Remote", "البعيد"),
        ("Remote Computer", "الحاسب البعيد"),
        ("Local Computer", "الحاسب المحلي"),
        ("Confirm Delete", "تأكيد الحذف"),
        ("Delete", "حذف"),
        ("Properties", "الخصائص"),
        ("Multi Select", "اختيار متعدد"),
        ("Select All", "تحديد الكل"),
        ("Unselect All", "الغاء تحديد الكل"),
        ("Empty Directory", "مجلد فارغ"),
        ("Not an empty directory", "مجلد غير فارغ"),
        ("Are you sure you want to delete this file?", "هل انت متأكد من أنك تريد حذف هذا الملف؟"),
        ("Are you sure you want to delete this empty directory?", "هل انت متأكد من أنك تريد حذف هذا المجلد؟"),
        ("Are you sure you want to delete the file of this directory?", "هل انت متأكد من أنك تريد حذف ملفات هذا المجلد؟"),
        ("Do this for all conflicts", "فعل هذا لكل التعارضات"),
        ("This is irreversible!", "لا يمكن التراجع عن هذا!"),
        ("Deleting", "جاري الحذف"),
        ("files", "ملفات"),
        ("Waiting", "انتظار"),
        ("Finished", "انتهى"),
        ("Speed", "السرعة"),
        ("Custom Image Quality", "جودة صورة مخصصة"),
        ("Privacy mode", "وضع الخصوصية"),
        ("Block user input", "حجم ادخال المستخدم"),
        ("Unblock user input", "الغاء حجب ادخال المستخدم"),
        ("Adjust Window", "ضبط النافذة"),
        ("Original", "الاصلي"),
        ("Shrink", "تقليص"),
        ("Stretch", "تمديد"),
        ("Scrollbar", "شريط التمرير"),
        ("ScrollAuto", "التمرير التلقائي"),
        ("Good image quality", "دقة صورة جيدة"),
        ("Balanced", "متوازن"),
        ("Optimize reaction time", "تحسين وقت رد الفعل"),
        ("Custom", "مخصص"),
        ("Show remote cursor", "عرض مؤشر الجهاز البعيد"),
        ("Show quality monitor", "عرض مراقب الجودة"),
        ("Disable clipboard", "تعطيل الحافظة"),
        ("Lock after session end", "القفل بعد نهاية هذه الجلسة"),
        ("Insert", "ادخال"),
        ("Insert Lock", "قفل الادخال"),
        ("Refresh", "تحديث"),
        ("ID does not exist", "المعرف غير موجود"),
        ("Failed to connect to rendezvous server", "فشل الاتصال بخادم rendezvous"),
        ("Please try later", "الرجاء المحاولة لاحقا"),
        ("Remote desktop is offline", "سطح المكتب البعيد غير متصل"),
        ("Key mismatch", "المفتاح غير متطابق"),
        ("Timeout", "نفذ الوقت"),
        ("Failed to connect to relay server", "فشل الاتصال بالخادم الوسيط"),
        ("Failed to connect via rendezvous server", "فشل الاتصال عير خادم rendezvous"),
        ("Failed to connect via relay server", "فشل الاتصال عبر الخادم الوسيط"),
        ("Failed to make direct connection to remote desktop", "فشل في اجراء اتصال مباشر لسطح المكتب البعيد"),
        ("Set Password", "ضبط كلمة المرور"),
        ("OS Password", "كلمة مرور نظام التشغيل"),
        ("install_tip", "بسبب صلاحيات تحكم حساب المستخدم. RustDesk قد لا يعمل بشكل صحيح في جهة البعيد في بعض الحالات. لتفادي ذلك. الرجاء الضغط على الزر ادناه لتثبيت RustDesk في جهازك."),
        ("Click to upgrade", "اضغط للارتقاء"),
        ("Click to download", "اضغط للتنزيل"),
        ("Click to update", "ضغط للتحديث"),
        ("Configure", "تهيئة"),
        ("config_acc", "لتتمكن من التحكم بسطح مكتبك البعيد, تحتاج الى منح RustDesk اذونات \"امكانية الوصول\"."),
        ("config_screen", "لتتمكن من الوصول الى سطح مكتبك البعيد, تحتاج الى منح RustDesk اذونات \"تسجيل الشاشة\"."),
        ("Installing ...", "جاري التثبيت..."),
        ("Install", "تثبيت"),
        ("Installation", "التثبيت"),
        ("Installation Path", "مسار التثبيت"),
        ("Create start menu shortcuts", "انشاء اختصارات قائمة ابداء"),
        ("Create desktop icon", "انشاء اختصار سطح المكتب"),
        ("agreement_tip", "بمجرد البدء بالتثبيت, فانت قد قبلت اتفاقية الترخيص."),
        ("Accept and Install", "الموافقة والتثبيت"),
        ("End-user license agreement", "اتفاقية ترخيص المستخدم النهائي"),
        ("Generating ...", "جاري الانشاء..."),
        ("Your installation is lower version.", "انت تحاول تثبيت نسخة قديمة."),
        ("not_close_tcp_tip", "لا تغلق هذه النافذة اثناء استخدامك للنفق"),
        ("Listening ...", "جاري الاستماع..."),
        ("Remote Host", "المستضيف البعيد"),
        ("Remote Port", "منفذ المستضيف البعيد"),
        ("Action", "فعل"),
        ("Add", "اضافة"),
        ("Local Port", "المنفذ المحلي"),
        ("Local Address", "العنوان المحلي"),
        ("Change Local Port", "تغيير المنفذ المحلي"),
        ("setup_server_tip", "لاتصال اسرع, الرجاء اعداد خادم خاص بك"),
        ("Too short, at least 6 characters.", "قصير جدا. يجب ان يكون على الاقل 6 خانات"),
        ("The confirmation is not identical.", "التأكيد غير متطابق"),
        ("Permissions", "الاذونات"),
        ("Accept", "قبول"),
        ("Dismiss", "صرف"),
        ("Disconnect", "قطع الاتصال"),
        ("Enable file copy and paste", "السماح بالنسخ واللصق"),
        ("Connected", "متصل"),
        ("Direct and encrypted connection", "اتصال مباشر مشفر"),
        ("Relayed and encrypted connection", "اتصال غير مباشر مشفر"),
        ("Direct and unencrypted connection", "اتصال مباشر غير مشفر"),
        ("Relayed and unencrypted connection", "اتصال غير مباشر غير مشفر"),
        ("Enter Remote ID", "ادخل المعرف البعيد"),
        ("Enter your password", "ادخل كلمة المرور"),
        ("Logging in...", "جاري تسجيل الدخول..."),
        ("Enable RDP session sharing", "تفعيل مشاركة الجلسة باستخدام RDP"),
        ("Auto Login", "تسجيل دخول تلقائي"),
        ("Enable direct IP access", "تفعيل الوصول المباشر لعنوان IP"),
        ("Rename", "اعادة تسمية"),
        ("Space", "مساحة"),
        ("Create desktop shortcut", "انشاء اختصار سطح مكتب"),
        ("Change Path", "تغيير المسار"),
        ("Create Folder", "انشاء مجلد"),
        ("Please enter the folder name", "الرجاء ادخال اسم المجلد"),
        ("Fix it", "اصلحه"),
        ("Warning", "تحذير"),
        ("Login screen using Wayland is not supported", "تسجيل الدخول باستخدام Wayland غير مدعوم"),
        ("Reboot required", "يجب اعادة التشغيل"),
        ("Unsupported display server", "خادم العرض غير مدعوم"),
        ("x11 expected", "x11 متوقع"),
        ("Port", "المنفذ"),
        ("Settings", "الاعدادات"),
        ("Username", "اسم المستخدم"),
        ("Invalid port", "منفذ خاطئ"),
        ("Closed manually by the peer", "اغلاق يدويا بواسطة القرين"),
        ("Enable remote configuration modification", "تفعيل تعديل اعدادات البعيد"),
        ("Run without install", "تشغيل بدون تثبيت"),
        ("Connect via relay", "الاتصال عبر وسيط"),
        ("Always connect via relay", "الاتصال باستخدام وسيط دائما"),
        ("whitelist_tip", "فقط قائمة الـ IP البيضاء تستطيع الوصول لي"),
        ("Login", "تسجيل الدخول"),
        ("Verify", "تأكيد"),
        ("Remember me", "تذكرني"),
        ("Trust this device", "الوثوق بهذا الجهاز"),
        ("Verification code", "رمز التحقق"),
        ("verification_tip", "رمز التحقق ارسل الى بريدك الالكتروني المسجل. ادخل رمز التحقق للاستمرار بتسجيل الدخول."),
        ("Logout", "تسجيل الخروج"),
        ("Tags", "العلامات"),
        ("Search ID", "البحث المعرف"),
        ("whitelist_sep", "مفصولة بفاصلة او فاصلة منقوطة او سطر جديد"),
        ("Add ID", "اضافة معرف"),
        ("Add Tag", "اضافة علامة"),
        ("Unselect all tags", "عدم تحديد كل العلامات"),
        ("Network error", "خطأ شبكة"),
        ("Username missed", "اسم المستخدم مفقود"),
        ("Password missed", "كلمة المرور مفقودة"),
        ("Wrong credentials", "اسم مستخدم او كلمة مرور خاطئة"),
        ("The verification code is incorrect or has expired", "رمز التحقق غير صحيح او منتهي"),
        ("Edit Tag", "تحرير علامة"),
        ("Forget Password", "عدم تذكر كلمة المرور"),
        ("Favorites", "المفضلة"),
        ("Add to Favorites", "اضافة للمفضلة"),
        ("Remove from Favorites", "ازالة من المفضلة"),
        ("Empty", "فارغ"),
        ("Invalid folder name", "اسم المجلد غير صحيح"),
        ("Socks5 Proxy", "وكيل Socks5"),
        ("Discovered", "المكتشفة"),
        ("install_daemon_tip", "للبدء مع بدء تشغيل النظام. تحتاج الى تثبيت خدمة النظام."),
        ("Remote ID", "المعرف البعيد"),
        ("Paste", "لصق"),
        ("Paste here?", "لصق هنا؟"),
        ("Are you sure to close the connection?", "هل انت متاكد من انك تريد اغلاق هذا الاتصال؟"),
        ("Download new version", "تنويل نسخة جديدة"),
        ("Touch mode", "وضع اللمس"),
        ("Mouse mode", "وضع الفأرة"),
        ("One-Finger Tap", "لم اصبع واحد"),
        ("Left Mouse", "الفأرة اليسرى"),
        ("One-Long Tap", "لمسة واحدة طويلة"),
        ("Two-Finger Tap", "لمس اصبعين"),
        ("Right Mouse", "الفأرة اليمنى"),
        ("One-Finger Move", "نقل الاصبع الواحد"),
        ("Double Tap & Move", "لمستان ونقل"),
        ("Mouse Drag", "سحب الفأرة"),
        ("Three-Finger vertically", "ثلاث اصابع افقيا"),
        ("Mouse Wheel", "عجلة الفارة"),
        ("Two-Finger Move", "نقل الاصبعين"),
        ("Canvas Move", ""),
        ("Pinch to Zoom", "قرصة للتكبير"),
        ("Canvas Zoom", ""),
        ("Reset canvas", ""),
        ("No permission of file transfer", "لا يوجد اذن نقل الملف"),
        ("Note", "ملاحظة"),
        ("Connection", "الاتصال"),
        ("Share Screen", "مشاركة الشاشة"),
        ("Chat", "محادثة"),
        ("Total", "الاجمالي"),
        ("items", "عناصر"),
        ("Selected", "محدد"),
        ("Screen Capture", "لقط الشاشة"),
        ("Input Control", "تحكم الادخال"),
        ("Audio Capture", "لقط الصوت"),
        ("File Connection", "اتصال الملف"),
        ("Screen Connection", "اتصال الشاشة"),
        ("Do you accept?", "هل تقبل؟"),
        ("Open System Setting", "فتح اعدادات النظام"),
        ("How to get Android input permission?", "كيف تحصل على اذن الادخال في اندرويد؟"),
        ("android_input_permission_tip1", "لكي يتمكن جهاز بعيد من التحكم بجهازك الـ Android عن طريق الفارة أو اللمس، يلزمك السماح لـ RustDesk باستخدام خدمة \"إمكانية الوصول\"."),
        ("android_input_permission_tip2", "يرجى الانتقال إلى صفحة إعدادات النظام التالية، والعثور على [الخدمات المثبتة]، وتشغيل خدمة [RustDesk Input]."),
        ("android_new_connection_tip", "تم استلام طلب تحكم جديد، حيث يريد التحكم بجهازك الحالي."),
        ("android_service_will_start_tip", "تشغيل \"لقط الشاشة\" سيبدأ الخدمة تلقائيا، حيث سيسمح للاجهزة الاخرى بطلب الاتصال مع جهازك."),
        ("android_stop_service_tip", "اغلاق الخدمة سيغلق جميع الاتصالات القائمة."),
        ("android_version_audio_tip", "نسخة اندرويد الحالية لا تدعم خدمة لقط الصوت، الرجاء الترقية الى نسخة 10 او أحدث."),
        ("android_start_service_tip", "اضغط تشغيل الخدمة او فعل صلاحية لقط الشاشة لبدء خدمة مشاركة الشاشة."),
        ("android_permission_may_not_change_tip", "الاذونات الاتصالات القائمة قد لا تتغير مباشرة الا بعد اعادة الاتصال."),
        ("Account", "الحساب"),
        ("Overwrite", "استبدال"),
        ("This file exists, skip or overwrite this file?", "الملف موجود, هل تريد التجاوز او الاستبدال؟"),
        ("Quit", "خروج"),
        ("Help", "مساعدة"),
        ("Failed", "فشل"),
        ("Succeeded", "نجاح"),
        ("Someone turns on privacy mode, exit", "شخص ما فعل وضع الخصوصية, خروج"),
        ("Unsupported", "غير مدعوم"),
        ("Peer denied", "القرين رفض"),
        ("Please install plugins", "الرجاء تثبيت الاضافات"),
        ("Peer exit", "خروج القرين"),
        ("Failed to turn off", "فشل ايقاف التشغيل"),
        ("Turned off", "مطفئ"),
        ("Language", "اللغة"),
        ("Keep RustDesk background service", "ابق خدمة RustDesk تعمل في الخلفية"),
        ("Ignore Battery Optimizations", "تجاهل تحسينات البطارية"),
        ("android_open_battery_optimizations_tip", "اذا اردت تعطيل هذه الميزة, الرجاء الذهاب الى صفحة اعدادات تطبيق RustDesk, ابحث عن البطارية, الغ تحديد غير مقيد"),
        ("Start on boot", "البدء عند تشغيل النظام"),
        ("Start the screen sharing service on boot, requires special permissions", "تشغيل خدمة مشاركة الشاشة عند بدء تشغيل النظام, يحتاج الى اذونات خاصة"),
        ("Connection not allowed", "الاتصال غير مسموح"),
        ("Legacy mode", "الوضع التقليدي"),
        ("Map mode", "وضع الخريطة"),
        ("Translate mode", "وضع الترجمة"),
        ("Use permanent password", "استخدام كلمة مرور دائمة"),
        ("Use both passwords", "استخدام طريقتي كلمة المرور"),
        ("Set permanent password", "تعيين كلمة مرور دائمة"),
        ("Enable remote restart", "تفعيل اعداة تشغيل البعيد"),
        ("Restart remote device", "اعادة تشغيل الجهاز البعيد"),
        ("Are you sure you want to restart", "هل انت متاكد من انك تريد اعادة التشغيل؟"),
        ("Restarting remote device", "جاري اعادة تشغيل البعيد"),
        ("remote_restarting_tip", "الجهاز البعيد يعيد التشغيل. الرجاء اغلاق هذه الرسالة واعادة الاتصال باستخدام كلمة المرور الدائمة بعد فترة بسيطة."),
        ("Copied", "منسوخ"),
        ("Exit Fullscreen", "الخروج من ملئ الشاشة"),
        ("Fullscreen", "ملئ الشاشة"),
        ("Mobile Actions", "اجراءات الهاتف"),
        ("Select Monitor", "اختر شاشة"),
        ("Control Actions", "اجراءات التحكم"),
        ("Display Settings", "اعدادات العرض"),
        ("Ratio", "النسبة"),
        ("Image Quality", "جودة الصورة"),
        ("Scroll Style", "شكل التمرير"),
        ("Show Toolbar", "عرض شريط الادوات"),
        ("Hide Toolbar", "اخفاء شريط الادوات"),
        ("Direct Connection", "اتصال مباشر"),
        ("Relay Connection", "اتصال الوسيط"),
        ("Secure Connection", "اتصال آمن"),
        ("Insecure Connection", "اتصال غير آمن"),
        ("Scale original", "المقياس الأصلي"),
        ("Scale adaptive", "مقياس التكيف"),
        ("General", "عام"),
        ("Security", "الأمان"),
        ("Theme", "السمة"),
        ("Dark Theme", "سمة غامقة"),
        ("Light Theme", "سمة فاتحة"),
        ("Dark", "غامق"),
        ("Light", "فاتح"),
        ("Follow System", "نفس نظام التشغيل"),
        ("Enable hardware codec", "تفعيل ترميز العتاد"),
        ("Unlock Security Settings", "فتح اعدادات الامان"),
        ("Enable audio", "تفعيل الصوت"),
        ("Unlock Network Settings", "فتح اعدادات الشبكة"),
        ("Server", "الخادم"),
        ("Direct IP Access", "وصول مباشر للـ IP"),
        ("Proxy", "الوكيل"),
        ("Apply", "تطبيق"),
        ("Disconnect all devices?", "قطع اتصال كل الاجهزة؟"),
        ("Clear", "مسح"),
        ("Audio Input Device", "جهاز ادخال الصوت"),
        ("Use IP Whitelisting", "استخدام قائمة الـ IP البيضاء"),
        ("Network", "الشبكة"),
        ("Pin Toolbar", "تثبيت شريط الادوات"),
        ("Unpin Toolbar", "الغاء تثبيت شريط الادوات"),
        ("Recording", "التسجيل"),
        ("Directory", "المسار"),
        ("Automatically record incoming sessions", "تسجيل الجلسات القادمة تلقائيا"),
        ("Change", "تغيير"),
        ("Start session recording", "بدء تسجيل الجلسة"),
        ("Stop session recording", "ايقاف تسجيل الجلسة"),
        ("Enable recording session", "تفعيل تسجيل الجلسة"),
        ("Enable LAN discovery", "تفعيل اكتشاف الشبكة المحلية"),
        ("Deny LAN discovery", "رفض اكتشاف الشبكة المحلية"),
        ("Write a message", "اكتب رسالة"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", "الرجاء انتظار تاكيد تحكم حساب المستخدم..."),
        ("elevated_foreground_window_tip", "النافذة الحالية لسطح المكتب البعيد تحتاج صلاحية اعلى لتعمل, لذلك لن تستطيع استخدام الفارة ولوحة المفاتيح مؤقتا. تستطيع انت تطلب من المستخدم البعيد تصغير النافذة الحالية, او ضفط زر الارتقاء في نافذة ادارة الاتصال. لتفادي هذة المشكلة من المستحسن تثبيت البرنامج في الجهاز البعيد."),
        ("Disconnected", "مفصول"),
        ("Other", "اخرى"),
        ("Confirm before closing multiple tabs", "التاكيد قبل اغلاق السنة عديدة"),
        ("Keyboard Settings", "اعدادات لوحة المفاتيح"),
        ("Full Access", "وصول كامل"),
        ("Screen Share", "مشاركة الشاشة"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland يتطلب نسخة ابونتو 21.04 او اعلى."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland يتطلب نسخة اعلى من توزيعة لينكس. الرجاء تجربة سطح مكتب X11 او غير نظام تشغيلك."),
        ("JumpLink", "رابط القفز"),
        ("Please Select the screen to be shared(Operate on the peer side).", "الرجاء اختيار شاشة لمشاركتها (تعمل على جانب القرين)."),
        ("Show RustDesk", "عرض RustDesk"),
        ("This PC", "هذا الحاسب"),
        ("or", "او"),
        ("Continue with", "متابعة مع"),
        ("Elevate", "ارتقاء"),
        ("Zoom cursor", "تكبير المؤشر"),
        ("Accept sessions via password", "قبول الجلسات عبر كلمة المرور"),
        ("Accept sessions via click", "قبول الجلسات عبر الضغط"),
        ("Accept sessions via both", "قبول الجلسات عبر الاثنين"),
        ("Please wait for the remote side to accept your session request...", "الرجاء الانتظار حتى يقبل الطرف البعيد طلب الجلسة..."),
        ("One-time Password", "كلمة مرور لمرة واحدة"),
        ("Use one-time password", "استخدام كلمة مرور لمرة واحدة"),
        ("One-time password length", "طول كلمة مرور لمرة واحدة"),
        ("Request access to your device", "طلب الوصول إلى جهازك"),
        ("Hide connection management window", "اخفاء نافذة ادارة الاتصال"),
        ("hide_cm_tip", "السماح بالاخفاء فقط في حالة قبول الجلسات عبر كلمة المرور واستخدام كلمة المرور الدائمة"),
        ("wayland_experiment_tip", "دعم Wayland لازال في المرحلة التجريبية. الرجاء استخدام X11 اذا اردت وصول كامل."),
        ("Right click to select tabs", "الضغط بالزر الايمن لتحديد الالسنة"),
        ("Skipped", "متجاوز"),
        ("Add to address book", "اضافة الى كتاب العناوين"),
        ("Group", "مجموعة"),
        ("Search", "بحث"),
        ("Closed manually by web console", "اغلق يدويا عبر طرفية الويب"),
        ("Local keyboard type", "توع لوحة المفاتيح المحلية"),
        ("Select local keyboard type", "اختر نوع لوحة المفاتيح الملحية"),
        ("software_render_tip", "اذا كنت تستخدم بطاقة رسوميات Nvidia تحت لينكس والشاشة البعيد تغلق مباشرة بعد الاتصال, قم بالتبديل الى تعريفات Nouveau مفتوحة المصدر واختيار الترميز البرمجي قد يساعد. اعادة تشغيل البرناج مطلوبة."),
        ("Always use software rendering", "استخدام الترميز البرمجي دائما"),
        ("config_input", "لتتمكن من التحكم بسطح المكتب البعيد. يجب من RustDesk اذونات \"مراقبة المدخلات\"."),
        ("config_microphone", "لتتمكن من التحدث. يجب منح RustDesk اذونات \"تسجيل الصوت\"."),
        ("request_elevation_tip", "اطلب الارتقاء في حالة وجود شخص في الطرف الاخر."),
        ("Wait", "انتظر"),
        ("Elevation Error", "خطأ الارتقاء"),
        ("Ask the remote user for authentication", "اسأل المستخدم البعيد التوثيق"),
        ("Choose this if the remote account is administrator", "اختر اذا كان الحساب البعيد مدير للنظام"),
        ("Transmit the username and password of administrator", "انقل اسم المستخدم وكلمة مرور مدير النظام"),
        ("still_click_uac_tip", "لازال يحتاج المستخدم البعيد للضغط على موافق في نافذة تحكم حساب المستخدم في RustDesk الذي يعمل."),
        ("Request Elevation", "طلب ارتقاء"),
        ("wait_accept_uac_tip", "الرجاء انتظار المستخدم البعيد حتى يوافق على طلب تحكم حساب المستخدم."),
        ("Elevate successfully", "الارتقاء بنجاح"),
        ("uppercase", "حرف كبير"),
        ("lowercase", "حرف صغير"),
        ("digit", "رقم"),
        ("special character", "رمز"),
        ("length>=8", "الطول 8 على الاقل"),
        ("Weak", "ضعيف"),
        ("Medium", "متوسط"),
        ("Strong", "قوي"),
        ("Switch Sides", "تبديل الاماكن"),
        ("Please confirm if you want to share your desktop?", "الرجاء التاكيد على انك تريد مشاركة سطح مكتبك؟"),
        ("Display", "العرض"),
        ("Default View Style", "شكل العرض الافتراضي"),
        ("Default Scroll Style", "شكل التمرير الافتراضي"),
        ("Default Image Quality", "دقة الصورة الافتراضية"),
        ("Default Codec", "الترميز الاقتراضي"),
        ("Bitrate", "معدل البت"),
        ("FPS", "مشهد في الثانية"),
        ("Auto", "تلقائي"),
        ("Other Default Options", "الخيارات الافتراضية الاخرى"),
        ("Voice call", "مكالمة صوتية"),
        ("Text chat", "دردشة نصية"),
        ("Stop voice call", "ايقاف المكالمة الصوتية"),
        ("relay_hint_tip", "قد لا يكون ممكن الاتصال مباشرة. يمكنك محاولة الاتصال عبر وسيط. ايضا اذا اردت استخدام وسيط لمحاولتك الاولى اضف لاحقة \"/r\" الى المعرف او اختر \"الاتصال باستخدام وسيط دائما\" في بطاقة الجلسات الحديثة ان وجدت."),
        ("Reconnect", "اعادة الاتصال"),
        ("Codec", "الترميز"),
        ("Resolution", "الدقة"),
        ("No transfers in progress", "لا توجد عمليات نقل حاليا"),
        ("Set one-time password length", "تعيين طول كلمة مرور المرة الواحدة"),
        ("RDP Settings", "اعدادات RDP"),
        ("Sort by", "ترتيب حسب"),
        ("New Connection", "اتصال جديد"),
        ("Restore", "استعادة"),
        ("Minimize", "تصغير"),
        ("Maximize", "تكبير"),
        ("Your Device", "جهازك"),
        ("empty_recent_tip", "للاسف. لا توجد جلسات حديثة\nحان الوقت للتخطيط لواحدة جديدة."),
        ("empty_favorite_tip", "لا يوجد اقران مفضلين حتى الان؟\nحسنا لنبحث عن شخص للاتصال معه ومن ثم اضافته للمفضلة."),
        ("empty_lan_tip", "اه لا, يبدو انك لم تكتشف اي قرين بعد."),
        ("empty_address_book_tip", "يا عزيزي, يبدو انه لايوجد حاليا اي اقران في كتاب العناوين."),
        ("eg: admin", "مثلا: admin"),
        ("Empty Username", "اسم مستخدم فارغ"),
        ("Empty Password", "كلمة مرور فارغة"),
        ("Me", "انا"),
        ("identical_file_tip", "هذا الملف مطابق لملف موجود عن القرين."),
        ("show_monitors_tip", "عرض الشاشات في شريط الادوات"),
        ("View Mode", "وضع العرض"),
        ("login_linux_tip", "تحتاج الى تسجيل الدخول حساب لينكس البعيد وتفعيل جلسة سطح مكتب X"),
        ("verify_rustdesk_password_tip", "تحقق من كلمة مرور RustDesk"),
        ("remember_account_tip", "تذكر هذا الحساب"),
        ("os_account_desk_tip", "هذا الحساب مستخدم لتسجيل الدخول الى سطح المكتب البعيد وتفعيل الجلسة"),
        ("OS Account", "حساب نظام التشغيل"),
        ("another_user_login_title_tip", "مستخدم اخر مسجل دخول حاليا"),
        ("another_user_login_text_tip", "قطع الاتصال"),
        ("xorg_not_found_title_tip", "Xorg غير موجود"),
        ("xorg_not_found_text_tip", "الرجاء تثبيت Xorg"),
        ("no_desktop_title_tip", "لا يتوفر سطح مكتب"),
        ("no_desktop_text_tip", "الرجاء تثبيت سطح مكتب GNOME"),
        ("No need to elevate", "لا حاجة للارتقاء"),
        ("System Sound", "صوت النظام"),
        ("Default", "الافتراضي"),
        ("New RDP", "RDP جديد"),
        ("Fingerprint", "البصمة"),
        ("Copy Fingerprint", "نسخ البصمة"),
        ("no fingerprints", "لا توجد بصمات اصابع"),
        ("Select a peer", "اختر قرين"),
        ("Select peers", "اختر الاقران"),
        ("Plugins", "الاضافات"),
        ("Uninstall", "الغاء التثبيت"),
        ("Update", "تحديث"),
        ("Enable", "تفعيل"),
        ("Disable", "تعطيل"),
        ("Options", "الخيارات"),
        ("resolution_original_tip", "الدقة الأصلية"),
        ("resolution_fit_local_tip", "تناسب الدقة المحلية"),
        ("resolution_custom_tip", "دقة مخصصة"),
        ("Collapse toolbar", "طي شريط الادوات"),
        ("Accept and Elevate", "قبول وارتقاء"),
        ("accept_and_elevate_btn_tooltip", "قبول الاتصال وارتقاء صلاحيات التحكم بصلاحيات المستخدم."),
        ("clipboard_wait_response_timeout_tip", "انتهى وقت الانتظار لنسخ الرد."),
        ("Incoming connection", "اتصال قادم"),
        ("Outgoing connection", "اتصال مغادر"),
        ("Exit", "خروج"),
        ("Open", "فتح"),
        ("logout_tip", "هل انت متاكد من انك تريد تسجيل الخروج"),
        ("Service", "الخدمة"),
        ("Start", "تشغيل"),
        ("Stop", "ايقاف"),
        ("exceed_max_devices", "لقد وصلت الحد الأقصى لعدد الاجهزة التي يمكن دارتها."),
        ("Sync with recent sessions", "المزامنة مع الجلسات الحديثة"),
        ("Sort tags", "ترتيب العلامات"),
        ("Open connection in new tab", "فتح اتصال في لسان جديد"),
        ("Move tab to new window", "نقل اللسان الى نافذة جديدة"),
        ("Can not be empty", "لا يمكن ان يكون فارغ"),
        ("Already exists", "موجود مسبقا"),
        ("Change Password", "تغيير كلمة المرور"),
        ("Refresh Password", "تحديث كلمة المرور"),
        ("ID", "المعرف"),
        ("Grid View", "عرض شبكي"),
        ("List View", "رعض قائمة"),
        ("Select", "اختيار"),
        ("Toggle Tags", "تفعيل/تعطيل العلامات"),
        ("pull_ab_failed_tip", "فشل تحديث كتاب العناوين"),
        ("push_ab_failed_tip", "فشل مزامنة كتاب العناوين مع الخادم"),
        ("synced_peer_readded_tip", "الاجهزة الموجودة في الجلسات الحديثة سيتم مزامنتها مع كتاب العناوين"),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("config-sas-tip", ""),
        ("sas-enabled-tip", ""),
        ("sas-disabled-tip", ""),
    ].iter().cloned().collect();
}
