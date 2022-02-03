import React from "react";
import { BsShieldFillCheck } from "react-icons/bs";
import { BiSearchAlt } from "react-icons/bi";
import { RiHeart2Fill } from "react-icons/ri";

const ServiceCard = ({ color, title, icon, subtitle }) => (
  <div className="flex flex-row justify-start items-start white-glassmorphism p-3 m-2 cursor-pointer hover:shadow-xl">
    <div className={`w-10 h-10 rounded-full flex justify-center items-center ${color}`}>
      {icon}
    </div>
    <div className="ml-5 flex flex-col flex-1">
      <h3 className="mt-2 text-white text-lg">{title}</h3>
      <p className="mt-1 text-white text-sm md:w-9/12">
        {subtitle}
      </p>
    </div>
  </div>
);

const Services = () => (
  <div className="flex w-full justify-center items-center gradient-bg-services">
    <div className="flex mf:flex-row flex-col items-center justify-between md:p-20 py-12 px-4">
      <div className="flex-1 flex flex-col justify-start items-start">
        <h1 className="text-white text-3xl sm:text-5xl py-2 text-gradient ">
          Everything you need
          <br />
          to accept
          <br />
          cryptocurrency Payments 
        </h1>
        <p className="text-left my-2 text-white font-light md:w-9/12 w-11/12 text-base">
        </p>
      </div>

      <div className="flex-1 flex flex-col justify-start items-center">
        <ServiceCard
          color="bg-[#2952E3]"
          title="Security gurantee"
          icon={<BsShieldFillCheck fontSize={21} className="text-white" />}
          subtitle="Security is guranteed. We always maintain privacy and maintain the quality of our product"
        />
        <ServiceCard
          color="bg-[#8945F8]"
          title="Best in quality services"
          icon={<BiSearchAlt fontSize={21} className="text-white" />}
          subtitle="We are commited to provide the best in quality services to Merchants as well as users"
        />
        <ServiceCard
          color="bg-[#F84550]"
          title="Fastest transactions and low cost"
          icon={<RiHeart2Fill fontSize={21} className="text-white" />}
          subtitle="With Solana X Serum unprecedented speed and low transaction costs to decentralized finance"
        />
      </div>
    </div>
  </div>
);

export default Services;
