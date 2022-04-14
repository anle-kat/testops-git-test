<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>main_DocumentationThe Selenium Browser Auto_7262de</name>
   <tag></tag>
   <elementGuidId>d46488e4-e7b8-4b91-8f14-4ebf34ccd7e3</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Print entire section'])[1]/following::main[1]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>main.col-12.col-md-9.col-xl-8.pl-md-5</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>main</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>col-12 col-md-9 col-xl-8 pl-md-5</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>role</name>
      <type>Main</type>
      <value>main</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>DocumentationThe Selenium Browser Automation ProjectSelenium is an umbrella project for a range of tools and libraries
that enable and support the automation of web browsers.It provides extensions to emulate user interaction with browsers,
a distribution server for scaling browser allocation,
and the infrastructure for implementations of the
W3C WebDriver specification
that lets you write interchangeable code for all major web browsers.This project is made possible by volunteer contributors
who have put in thousands of hours of their own time,
and made the source code
freely available
for anyone to use, enjoy, and improve.Selenium brings together browser vendors, engineers, and enthusiasts
to further an open discussion around automation of the web platform.
The project organises an annual conference
to teach and nurture the community.At the core of Selenium is WebDriver,
an interface to write instruction sets that can be run interchangeably in many
browsers. Once you’ve installed everything, only a few lines of code get you inside
a browser. You can find a more comprehensive example in Writing your first Selenium scriptJavaPythonCSharpRubyJavaScriptKotlinpackage dev.selenium.hello;

import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

public class HelloSelenium {
    public static void main(String[] args) {
        WebDriver driver = new ChromeDriver();

        driver.get(&quot;https://selenium.dev&quot;);

        driver.quit();
    }
}
Copy

Check code on GitHubfrom selenium import webdriver


driver = webdriver.Chrome()

driver.get(&quot;http://selenium.dev&quot;)

driver.quit()Copy

Check code on GitHubusing OpenQA.Selenium.Chrome;

namespace SeleniumDocs.Hello
{
    public class HelloSelenium
    {
        public static void Main()
        {
            var driver = new ChromeDriver();
            
            driver.Navigate().GoToUrl(&quot;https://selenium.dev&quot;);
            
            driver.Quit();
        }
    }
}Copy

Check code on GitHubrequire 'selenium-webdriver'

driver = Selenium::WebDriver.for :chrome

driver.get 'https://selenium.dev'

driver.quit
Copy

Check code on GitHubconst {Builder} = require('selenium-webdriver');

(async function helloSelenium() {
    let driver = await new Builder().forBrowser('chrome').build();

    await driver.get('https://selenium.dev');

    await driver.quit();
})();Copy

Check code on GitHubpackage dev.selenium.hello

import org.openqa.selenium.chrome.ChromeDriver

fun main() {
    val driver = ChromeDriver()

    driver.get(&quot;https://selenium.dev&quot;)

    driver.quit()
}
Copy

Check code on GitHubSee the Overview to check the different project
components and decide if Selenium is the right tool for you.You should continue on to Getting Started
to understand how you can install Selenium and successfully use it as a test
automation tool, and scaling simple tests like this to run in large, distributed
environments on multiple browsers, on several different operating systems.Selenium overviewIs Selenium for you? See an overview of the different project components.WebDriverWebDriver drives a browser natively, learn more about it.Selenium Grid 4Want to run tests in parallel across multiple machines? Then, Grid is for you.IE Driver ServerThe Internet Explorer Driver is a standalone server that implements the WebDriver specification.Selenium IDEThe Selenium IDE is a browser extension that records and plays back a user’s actions.Test PracticesSome guidelines and recommendations on testing from the Selenium project.LegacyDocumentation related to the legacy components of Selenium. Meant to be kept purely for historical reasons and not as a incentive to use deprecated components.About this documentationLast modified March 16, 2022: Linking examples on landing docs page (40b58137835)</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;no-js&quot;]/body[@class=&quot;td-section&quot;]/div[@class=&quot;container-fluid td-outer&quot;]/div[@class=&quot;td-main&quot;]/div[@class=&quot;row flex-xl-nowrap&quot;]/main[@class=&quot;col-12 col-md-9 col-xl-8 pl-md-5&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Print entire section'])[1]/following::main[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Create project issue'])[1]/following::main[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//main</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//main[(text() = concat(&quot;DocumentationThe Selenium Browser Automation ProjectSelenium is an umbrella project for a range of tools and libraries
that enable and support the automation of web browsers.It provides extensions to emulate user interaction with browsers,
a distribution server for scaling browser allocation,
and the infrastructure for implementations of the
W3C WebDriver specification
that lets you write interchangeable code for all major web browsers.This project is made possible by volunteer contributors
who have put in thousands of hours of their own time,
and made the source code
freely available
for anyone to use, enjoy, and improve.Selenium brings together browser vendors, engineers, and enthusiasts
to further an open discussion around automation of the web platform.
The project organises an annual conference
to teach and nurture the community.At the core of Selenium is WebDriver,
an interface to write instruction sets that can be run interchangeably in many
browsers. Once you’ve installed everything, only a few lines of code get you inside
a browser. You can find a more comprehensive example in Writing your first Selenium scriptJavaPythonCSharpRubyJavaScriptKotlinpackage dev.selenium.hello;

import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

public class HelloSelenium {
    public static void main(String[] args) {
        WebDriver driver = new ChromeDriver();

        driver.get(&quot;https://selenium.dev&quot;);

        driver.quit();
    }
}
Copy

Check code on GitHubfrom selenium import webdriver


driver = webdriver.Chrome()

driver.get(&quot;http://selenium.dev&quot;)

driver.quit()Copy

Check code on GitHubusing OpenQA.Selenium.Chrome;

namespace SeleniumDocs.Hello
{
    public class HelloSelenium
    {
        public static void Main()
        {
            var driver = new ChromeDriver();
            
            driver.Navigate().GoToUrl(&quot;https://selenium.dev&quot;);
            
            driver.Quit();
        }
    }
}Copy

Check code on GitHubrequire &quot; , &quot;'&quot; , &quot;selenium-webdriver&quot; , &quot;'&quot; , &quot;

driver = Selenium::WebDriver.for :chrome

driver.get &quot; , &quot;'&quot; , &quot;https://selenium.dev&quot; , &quot;'&quot; , &quot;

driver.quit
Copy

Check code on GitHubconst {Builder} = require(&quot; , &quot;'&quot; , &quot;selenium-webdriver&quot; , &quot;'&quot; , &quot;);

(async function helloSelenium() {
    let driver = await new Builder().forBrowser(&quot; , &quot;'&quot; , &quot;chrome&quot; , &quot;'&quot; , &quot;).build();

    await driver.get(&quot; , &quot;'&quot; , &quot;https://selenium.dev&quot; , &quot;'&quot; , &quot;);

    await driver.quit();
})();Copy

Check code on GitHubpackage dev.selenium.hello

import org.openqa.selenium.chrome.ChromeDriver

fun main() {
    val driver = ChromeDriver()

    driver.get(&quot;https://selenium.dev&quot;)

    driver.quit()
}
Copy

Check code on GitHubSee the Overview to check the different project
components and decide if Selenium is the right tool for you.You should continue on to Getting Started
to understand how you can install Selenium and successfully use it as a test
automation tool, and scaling simple tests like this to run in large, distributed
environments on multiple browsers, on several different operating systems.Selenium overviewIs Selenium for you? See an overview of the different project components.WebDriverWebDriver drives a browser natively, learn more about it.Selenium Grid 4Want to run tests in parallel across multiple machines? Then, Grid is for you.IE Driver ServerThe Internet Explorer Driver is a standalone server that implements the WebDriver specification.Selenium IDEThe Selenium IDE is a browser extension that records and plays back a user’s actions.Test PracticesSome guidelines and recommendations on testing from the Selenium project.LegacyDocumentation related to the legacy components of Selenium. Meant to be kept purely for historical reasons and not as a incentive to use deprecated components.About this documentationLast modified March 16, 2022: Linking examples on landing docs page (40b58137835)&quot;) or . = concat(&quot;DocumentationThe Selenium Browser Automation ProjectSelenium is an umbrella project for a range of tools and libraries
that enable and support the automation of web browsers.It provides extensions to emulate user interaction with browsers,
a distribution server for scaling browser allocation,
and the infrastructure for implementations of the
W3C WebDriver specification
that lets you write interchangeable code for all major web browsers.This project is made possible by volunteer contributors
who have put in thousands of hours of their own time,
and made the source code
freely available
for anyone to use, enjoy, and improve.Selenium brings together browser vendors, engineers, and enthusiasts
to further an open discussion around automation of the web platform.
The project organises an annual conference
to teach and nurture the community.At the core of Selenium is WebDriver,
an interface to write instruction sets that can be run interchangeably in many
browsers. Once you’ve installed everything, only a few lines of code get you inside
a browser. You can find a more comprehensive example in Writing your first Selenium scriptJavaPythonCSharpRubyJavaScriptKotlinpackage dev.selenium.hello;

import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;

public class HelloSelenium {
    public static void main(String[] args) {
        WebDriver driver = new ChromeDriver();

        driver.get(&quot;https://selenium.dev&quot;);

        driver.quit();
    }
}
Copy

Check code on GitHubfrom selenium import webdriver


driver = webdriver.Chrome()

driver.get(&quot;http://selenium.dev&quot;)

driver.quit()Copy

Check code on GitHubusing OpenQA.Selenium.Chrome;

namespace SeleniumDocs.Hello
{
    public class HelloSelenium
    {
        public static void Main()
        {
            var driver = new ChromeDriver();
            
            driver.Navigate().GoToUrl(&quot;https://selenium.dev&quot;);
            
            driver.Quit();
        }
    }
}Copy

Check code on GitHubrequire &quot; , &quot;'&quot; , &quot;selenium-webdriver&quot; , &quot;'&quot; , &quot;

driver = Selenium::WebDriver.for :chrome

driver.get &quot; , &quot;'&quot; , &quot;https://selenium.dev&quot; , &quot;'&quot; , &quot;

driver.quit
Copy

Check code on GitHubconst {Builder} = require(&quot; , &quot;'&quot; , &quot;selenium-webdriver&quot; , &quot;'&quot; , &quot;);

(async function helloSelenium() {
    let driver = await new Builder().forBrowser(&quot; , &quot;'&quot; , &quot;chrome&quot; , &quot;'&quot; , &quot;).build();

    await driver.get(&quot; , &quot;'&quot; , &quot;https://selenium.dev&quot; , &quot;'&quot; , &quot;);

    await driver.quit();
})();Copy

Check code on GitHubpackage dev.selenium.hello

import org.openqa.selenium.chrome.ChromeDriver

fun main() {
    val driver = ChromeDriver()

    driver.get(&quot;https://selenium.dev&quot;)

    driver.quit()
}
Copy

Check code on GitHubSee the Overview to check the different project
components and decide if Selenium is the right tool for you.You should continue on to Getting Started
to understand how you can install Selenium and successfully use it as a test
automation tool, and scaling simple tests like this to run in large, distributed
environments on multiple browsers, on several different operating systems.Selenium overviewIs Selenium for you? See an overview of the different project components.WebDriverWebDriver drives a browser natively, learn more about it.Selenium Grid 4Want to run tests in parallel across multiple machines? Then, Grid is for you.IE Driver ServerThe Internet Explorer Driver is a standalone server that implements the WebDriver specification.Selenium IDEThe Selenium IDE is a browser extension that records and plays back a user’s actions.Test PracticesSome guidelines and recommendations on testing from the Selenium project.LegacyDocumentation related to the legacy components of Selenium. Meant to be kept purely for historical reasons and not as a incentive to use deprecated components.About this documentationLast modified March 16, 2022: Linking examples on landing docs page (40b58137835)&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
