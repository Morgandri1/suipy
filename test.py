import suipy
import asyncio

async def main():
    print(suipy.getOwnedObjects("0x92e3c6069477e4cdec401d96f2db415ce1e846f0856986e8bc687fcee62f2aa1"))

asyncio.run(main())