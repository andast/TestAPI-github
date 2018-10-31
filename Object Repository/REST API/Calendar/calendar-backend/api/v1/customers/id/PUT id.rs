<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT id</name>
   <tag></tag>
   <elementGuidId>d1b95240-ad7b-46e7-9e49-37b138eb1b73</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;firstName\&quot;: \&quot;${firstName}\&quot;,\n\t\&quot;lastName\&quot;: \&quot;${lastName}\&quot;,\n\t\&quot;dateOfBirth\&quot;: \&quot;${dateOfBirth}\&quot;,\n\t\&quot;phone\&quot;: \&quot;${phone}\&quot;,\n\t\&quot;email\&quot;: \&quot;${email}\&quot;,\n\t\&quot;identificationNumber\&quot;: \&quot;${identificationNumber}\&quot;,\n\t\&quot;externalId\&quot;: \&quot;${externalId}\&quot;,\n\t\&quot;addressLine1\&quot;: \&quot;${addressLine1}\&quot;,\n\t\&quot;addressLine2\&quot;: \&quot;${addressLine2}\&quot;,\n\t\&quot;addressZip\&quot;: \&quot;${addressZip}\&quot;,\n\t\&quot;addressCity\&quot;: \&quot;${addressCity}\&quot;,\n\t\&quot;addressState\&quot;: \&quot;${addressState}\&quot;,\n\t\&quot;addressCountry\&quot;: \&quot;${addressCountry}\&quot;\n}&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${orchestraHost}/calendar-backend/api/v1/customers/${id}</restUrl>
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
      <id>8e66198a-3d67-45f0-9952-01f6797bf29f</id>
      <masked>false</masked>
      <name>basicAuth</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>38e9869a-1b05-4cb6-8b2b-3df5b49112bb</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'{}'</defaultValue>
      <description></description>
      <id>3d518d9f-9d56-4616-ba9a-94ce842e6553</id>
      <masked>false</masked>
      <name>body</name>
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
