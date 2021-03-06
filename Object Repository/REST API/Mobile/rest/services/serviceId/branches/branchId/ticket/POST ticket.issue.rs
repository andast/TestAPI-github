<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST ticket.issue</name>
   <tag></tag>
   <elementGuidId>63387e76-f157-43b7-9f1d-fd537dc05a78</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${body}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${basicAuth}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Referer</name>
      <type>Main</type>
      <value>${orchestraHost}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${orchestraHost}/qsystem/mobile/rest/services/${serviceId}/branches/${branchId}/ticket/issue?delay=${delay}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.orchestraHost</defaultValue>
      <description></description>
      <id>43fbcc0b-47de-4dcb-9571-15021e62f62c</id>
      <masked>false</masked>
      <name>orchestraHost</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.basicAuth</defaultValue>
      <description></description>
      <id>b1722694-f441-466b-ad05-1740e55b09db</id>
      <masked>false</masked>
      <name>basicAuth</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>647cca7b-2c94-48c2-bdbd-79df7d4cfd24</id>
      <masked>false</masked>
      <name>body</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>03f1a92f-9948-4607-95e9-8d40507db648</id>
      <masked>false</masked>
      <name>serviceId</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>32fe856b-c86b-4753-8a21-f0b3b5d4d4d1</id>
      <masked>false</masked>
      <name>branchId</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>f6138197-b990-44bb-a344-7d3af4ce69ee</id>
      <masked>false</masked>
      <name>delay</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
