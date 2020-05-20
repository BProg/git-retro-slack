pub fn create_parameters(exe_path: &str) -> String {
    format!(
        r#"
<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>com.ionostafi.gitretro</string>
    <key>Program</key>
    <string>{0}</string>
    <key>ProgramArguments</key>
    <array>
      <string>{0}</string>
    </array>
    <key>StartCalendarInterval</key>
    <array>
      <dict>
        <key>Weekday</key>
        <integer>1</integer>
        <key>Hour</key>
        <integer>10</integer>
        <key>Minute</key>
        <integer>00</integer>
      </dict>
      <dict>
        <key>Weekday</key>
        <integer>2</integer>
        <key>Hour</key>
        <integer>10</integer>
        <key>Minute</key>
        <integer>00</integer>
      </dict>
      <dict>
        <key>Weekday</key>
        <integer>3</integer>
        <key>Hour</key>
        <integer>10</integer>
        <key>Minute</key>
        <integer>00</integer>
      </dict>
      <dict>
        <key>Weekday</key>
        <integer>4</integer>
        <key>Hour</key>
        <integer>10</integer>
        <key>Minute</key>
        <integer>00</integer>
      </dict>
      <dict>
        <key>Weekday</key>
        <integer>5</integer>
        <key>Hour</key>
        <integer>10</integer>
        <key>Minute</key>
        <integer>00</integer>
      </dict>
    </array>
  </dict>
</plist>
"#, exe_path)
}